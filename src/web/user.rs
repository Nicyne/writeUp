//! Endpoints regarding user-objects and their manipulation

use actix_web::{get, put, delete, post, Responder, HttpRequest};
use actix_web::web::Path;

#[post("/user")]
pub async fn add_user(req: HttpRequest) -> impl Responder { //TODO implement
    format!("Request for adding a new user received")
}

#[get("/user/{user_id}")]
pub async fn get_user(req: HttpRequest, path: Path<String>) -> impl Responder { //TODO implement
    format!("Request for user(USERNAME='{}') received", path.into_inner())
}

#[put("/user/{user_id}")]
pub async fn update_user(req: HttpRequest, path: Path<String>) -> impl Responder { //TODO implement
    format!("Request for update of user(USERNAME='{}') received", path.into_inner())
}

#[delete("/user/{user_id}")]
pub async fn remove_user(req: HttpRequest, path: Path<String>) -> impl Responder { //TODO implement
    format!("Request for removal of user(USERNAME='{}') received", path.into_inner())
}