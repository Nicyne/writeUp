//! Endpoints regarding note-objects and their manipulation

use std::sync::Mutex;
use serde::Serialize;
use actix_web::{get, put, delete, post, Responder, HttpRequest, HttpResponse, web::{Data, Path}};
use mongodb::Database;
use crate::db_access::{AllowanceLevel, DBError, get_dbo_by_id, Note, NOTES, User, USER};
use crate::web::{get_user_id_from_request, error::AuthError};

// Response-Objects
#[derive(Serialize)]
struct NoteResponse {
    note: Note,
    allowance: AllowanceLevel
}

#[post("/note")]
pub async fn add_note(req: HttpRequest) -> impl Responder { //TODO implement
    format!("Request for adding a new note received")
}

#[get("/note/{note_id}")]
pub async fn get_note(path: Path<String>, req: HttpRequest, db: Data<Mutex<Database>>) -> impl Responder {
    let note_id = path.into_inner();
    // Check if the user has clearance to view this note
    match get_allow_level_for_note(&note_id, req, db.get_ref()).await {
        Ok(allowance) => {
            // Get note and return it
            match get_dbo_by_id::<Note>(NOTES, note_id, db.get_ref()).await {
                Ok(note) => HttpResponse::Ok().json(NoteResponse { note, allowance}),
                Err(DBError::NoDocumentFoundError) => HttpResponse::InternalServerError()
                    .json("reference to deleted note still exists in user-allowances"), //user has allowance for a nonexisting note
                Err(_) => HttpResponse::InternalServerError().finish() //unknown
            }
        }
        Err(e) => e.gen_response()
    }
}

#[put("/note/{note_id}")]
pub async fn update_note(req: HttpRequest, path: Path<u32>) -> impl Responder { //TODO implement
    format!("Request for update of note(ID={}) received", path.into_inner())
}

#[delete("/note/{note_id}")]
pub async fn remove_note(req: HttpRequest, path: Path<u32>) -> impl Responder { //TODO implement
    format!("Request for removal of note(ID={}) received", path.into_inner())
}

async fn get_allow_level_for_note(note_id: &str, req: HttpRequest, db: &Mutex<Database>) -> Result<AllowanceLevel, AuthError> {
    // Verify jwt
    match get_user_id_from_request(req) {
        Ok(user_id) => {
            // Extract the user
            match get_dbo_by_id::<User>(USER, user_id, db).await {
                Ok(user) => {
                    // Check if there is an allowance for this note
                    match user.allowances.iter().find(|all| all.note_id.eq(&note_id)) {
                        Some(allowance) => Ok(allowance.clone().level),
                        None => Err(AuthError::NoPermissionError)
                    }
                }
                Err(DBError::NoDocumentFoundError) => Err(AuthError::InvalidUserError),
                Err(_) => Err(AuthError::InternalServerError("could not retrieve user from database".to_string()))
            }
        }
        Err(e) => Err(e)
    }
}
