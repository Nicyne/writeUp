//! Endpoints regarding note-objects and their manipulation

use std::sync::Mutex;
use actix_web::{get, put, delete, post, Responder, HttpRequest, HttpResponse, web::{Data, Path}, web};
use mongodb::bson::doc;
use mongodb::Database;
use crate::db_access::{AllowanceLevel, DBError, get_dbo_by_id, insert_dbo, Note, NOTES, update_dbo_by_id, User, USER};
use crate::web::error::AuthError;
use crate::web::auth::get_user_from_request;
use crate::web::note::json_objects::{NoteRequest, NoteResponse};

// Response-/Request-Objects
mod json_objects {
    use serde::{Serialize, Deserialize};
    use crate::db_access::{AllowanceLevel, Note};

    #[derive(Deserialize)]
    pub struct NoteRequest {
        pub title: String,
        pub content: String,
        pub tags: Vec<String>
    }
    impl NoteRequest {
        pub fn to_note(self, owner_id: &str) -> Note {
            Note { title: self.title, content: self.content, owner_id: owner_id.to_string(), tags: self.tags }
        }
    }

    #[derive(Serialize)]
    pub struct NoteResponse {
        pub note_id: String,
        pub note: Note,
        pub allowance: AllowanceLevel
    }
}

#[post("/note")]
pub async fn add_note(req: HttpRequest, note_req: web::Json<NoteRequest>, db: Data<Mutex<Database>>) -> impl Responder {
    let note_req = note_req.into_inner();
    // Grab the user to add a note to
    match get_user_from_request(req, &db).await {
        Ok(user) => {
            // Add the new note to the db
            let note = note_req.to_note(&user._id);
            match insert_dbo::<Note>(NOTES, &note, db.get_ref()).await {
                Ok(res) => {
                    // Add an allowance to the user
                    let note_id = res.inserted_id.as_object_id().unwrap().to_string();
                    match update_dbo_by_id::<User>(USER, user._id,
                                                   doc! {"$push": {"allowances": {"note_id": &note_id, "level": "Owner"}}},
                                                   db.get_ref()).await {
                        Ok(res) => HttpResponse::Ok() // Return the created note
                            .json(NoteResponse { note_id, note, allowance: AllowanceLevel::Owner}), //TODO? Re-fetch object instead of putting together
                        Err(_) => AuthError::InternalServerError("note could not be linked to user-account".to_string()).gen_response() //unknown
                    }
                }
                Err(_) => AuthError::InternalServerError("note could not be saved to db".to_string()).gen_response() //unknown
            }
        }
        Err(e) => e.gen_response()
    }
}

#[get("/note/{note_id}")]
pub async fn get_note(path: Path<String>, req: HttpRequest, db: Data<Mutex<Database>>) -> impl Responder {
    let note_id = path.into_inner();
    // Check if the user has clearance to view this note
    match get_allow_level_for_note(&note_id, req, db.get_ref()).await {
        Ok(allowance) => {
            // Get note and return it
            match get_dbo_by_id::<Note>(NOTES, note_id.clone(), db.get_ref()).await {
                Ok(note) => HttpResponse::Ok().json(NoteResponse { note_id, note, allowance}),
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
    // Get the User making the request
    match get_user_from_request(req, db).await {
        Ok(user) => {
            // Check if there is an allowance for this note
            match user.allowances.iter().find(|all| all.note_id.eq(&note_id)) {
                Some(allowance) => Ok(allowance.clone().level),
                None => Err(AuthError::NoPermissionError)
            }
        }
        Err(e) => Err(e)
    }
}
