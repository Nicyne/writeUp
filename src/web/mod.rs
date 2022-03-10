//! REST-Endpoints and their logic

mod note;
mod user;
mod share;
mod error;
mod auth;

use std::sync::Mutex;
use serde::Serialize;
use actix_web::{get, HttpRequest, HttpResponse, Responder, web::{ServiceConfig, Data}};
use mongodb::{bson::doc, Database};
use crate::db_access::{AllowanceLevel, DBError, get_dbo_by_id, Note, NOTES};
use crate::web::auth::get_user_from_request;
use crate::web::error::AuthError;

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
async fn list_notes(req: HttpRequest, db: Data<Mutex<Database>>) -> impl Responder {
    // Define Response-Object
    #[derive(Serialize)]
    struct ReducedNoteResponse {
        note_id: String,
        title: String,
        tags: Vec<String>,
        allowance: AllowanceLevel
    }
    match get_user_from_request(req, db.get_ref()).await {
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
                    Err(DBError::NoDocumentFoundError) => return AuthError::InternalServerError(
                        format!("reference to deleted note(ID:{}) still exists in user-allowances",
                                allowance.note_id)).gen_response(), //user has allowance for a nonexisting note
                    Err(_) => {} //unknown
                }
            }
            // Return the compiled list of notes
            HttpResponse::Ok().json(response_vector)
        }
        Err(e) => e.gen_response()
    }
}
