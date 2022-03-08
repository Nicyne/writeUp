//! REST-Endpoints and their logic

mod note;
mod user;
mod share;
mod error;
mod auth;

use std::sync::Mutex;
use serde::Serialize;
use actix_web::{get, HttpResponse, Responder, web::{ServiceConfig, Data}};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use mongodb::{bson::doc, Database};
use crate::db_access::{AllowanceLevel, DBError, get_dbo_by_id, Note, NOTES, User, USER};
use crate::web::auth::get_user_id_from_jwt;

pub fn handler_config(cfg: &mut ServiceConfig) {
    // Add all special handler
    cfg.service(auth::authenticate)
        .service(list_notes);
    // Add all note-related handler
    cfg.service(note::add_note)
        .service(note::get_note)
        .service(note::update_note)
        .service(note::remove_note);
    // Add all user-related handler
    cfg.service(user::add_user)
        .service(user::get_user)
        .service(user::update_user)
        .service(user::remove_user);
    // Add all share-related handler
    cfg.service(share::create_relation)
        .service(share::get_relation_code)
        .service(share::update_share);
}

#[get("/notes")]
async fn list_notes(auth: BearerAuth, db: Data<Mutex<Database>>) -> impl Responder {
    // Define Response-Object
    #[derive(Serialize)]
    struct ReducedNoteResponse {
        note_id: String,
        title: String,
        tags: Vec<String>,
        allowance: AllowanceLevel
    }
    // Verify Credentials
    let user = get_user_id_from_jwt(auth.token());
    match user {
        Ok(user_id) => {
            // Get list of allowed notes
            match get_dbo_by_id::<User>(USER, user_id, db.get_ref()).await {
                Ok(user) => {
                    let mut response_vector = Vec::new();
                    for allowance in user.allowances {
                        // Read all allowed notes and create response-objects
                        match get_dbo_by_id::<Note>(NOTES, allowance.note_id.clone(), db.get_ref()).await {
                            Ok(note) => response_vector.push(ReducedNoteResponse {
                                note_id: allowance.note_id,
                                title: note.title,
                                tags: note.tags,
                                allowance: allowance.level
                            }),
                            Err(DBError::NoDocumentFoundError) => return HttpResponse::InternalServerError()
                                .json(format!("reference to deleted note(ID:{}) still exists in user-allowances", allowance.note_id)), //user has allowance for a nonexisting note
                            Err(_) => {} //unknown
                        }
                    }
                    // Return the compiled list of notes
                    HttpResponse::Ok().json(response_vector)
                }
                Err(DBError::NoDocumentFoundError) => HttpResponse::InternalServerError()
                    .json("User-Object is missing"), //there are credentials for a nonexisting user
                Err(_) => HttpResponse::InternalServerError().finish() //unknown
            }
        }
        Err(e) => e.gen_response()
    }
}
