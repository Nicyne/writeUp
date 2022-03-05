//! Endpoints regarding the sharing of notes and connecting of users

use actix_web::{get, put, post, Responder, HttpRequest};
use actix_web::web::Path;

#[post("/share")]
pub async fn create_relation(req: HttpRequest) -> impl Responder { //TODO implement
    format!("Request for adding a new relation received")
}

#[get("/share")]
pub async fn get_relation_code(req: HttpRequest) -> impl Responder { //TODO implement
    format!("Request for new relation-code received")
}

#[put("/share/{note_id}")]
pub async fn update_share(req: HttpRequest, path: Path<u32>) -> impl Responder { //TODO implement
    format!("Request for share-update of note(ID={}) received", path.into_inner())
}
