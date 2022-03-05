//! REST-Endpoints and their logic

mod note;
mod user;
mod share;

use actix_web::{get, HttpRequest, Responder, web};

pub fn handler_config(cfg: &mut web::ServiceConfig) {
    // Add all special handler
    cfg.service(authenticate)
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

#[get("/auth")]
async fn authenticate(req: HttpRequest) -> impl Responder { //TODO implement
    format!("Request for authentication received")
}

#[get("/notes")]
async fn list_notes(req: HttpRequest) -> impl Responder { //TODO implement
    format!("Request for listing all owned and shared notes received")
}
