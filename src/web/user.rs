//! Endpoints regarding user-objects and their manipulation

use std::sync::Mutex;
use actix_web::{get, put, delete, post, Responder, HttpRequest, HttpResponse};
use actix_web::web::{Data, Path};
use mongodb::Database;
use crate::web::auth::get_user_from_request;
use crate::web::user::json_objects::NoteRequest;

// Response-/Request-Objects
mod json_objects {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct NoteRequest {
        pub username: String
    }
}

#[post("/user")]
pub async fn add_user(req: HttpRequest) -> impl Responder { //TODO implement
    format!("Request for adding a new user received")
}

#[get("/user")]
pub async fn get_user(req: HttpRequest, db: Data<Mutex<Database>>) -> impl Responder {
    match get_user_from_request(req, &db).await {
        Ok(user) => HttpResponse::Ok().json(NoteRequest { username: user._id }),
        Err(e) => e.gen_response()
    }
}

#[put("/user/{user_id}")]
pub async fn update_user(req: HttpRequest, path: Path<String>) -> impl Responder { //TODO implement
    format!("Request for update of user(USERNAME='{}') received", path.into_inner())
}

#[delete("/user/{user_id}")]
pub async fn remove_user(req: HttpRequest, path: Path<String>) -> impl Responder { //TODO implement
    format!("Request for removal of user(USERNAME='{}') received", path.into_inner())
}