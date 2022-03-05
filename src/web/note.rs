//! Endpoints regarding note-objects and their manipulation

use actix_web::{get, put, delete, post, Responder, HttpRequest};
use actix_web::web::Path;

#[post("/note")]
pub async fn add_note(req: HttpRequest) -> impl Responder { //TODO implement
    format!("Request for adding a new note received")
}

#[get("/note/{note_id}")]
pub async fn get_note(req: HttpRequest, path: Path<u32>) -> impl Responder { //TODO implement
    format!("Request for note(ID={}) received", path.into_inner())
}

#[put("/note/{note_id}")]
pub async fn update_note(req: HttpRequest, path: Path<u32>) -> impl Responder { //TODO implement
    format!("Request for update of note(ID={}) received", path.into_inner())
}

#[delete("/note/{note_id}")]
pub async fn remove_note(req: HttpRequest, path: Path<u32>) -> impl Responder { //TODO implement
    format!("Request for removal of note(ID={}) received", path.into_inner())
}
