//! Endpoints regarding user-objects and their manipulation

use std::sync::Mutex;
use actix_web::{get, put, delete, post, Responder, HttpRequest, HttpResponse};
use actix_web::web::{Data, Path};
use mongodb::Database;
use crate::web::auth::get_user_from_request;
use crate::web::user::json_objects::UserResponse;

// Response-/Request-Objects
/// Structs modelling the request- and response-bodies
mod json_objects {
    use serde::Serialize;

    /// Body of a response containing user-information
    #[derive(Serialize)]
    pub struct UserResponse {
        /// The username
        pub username: String
    }
}

#[post("/user")]
pub async fn add_user(req: HttpRequest) -> impl Responder { //TODO implement
    format!("Request for adding a new user received")
}

/// ENDPOINT: Returns the currently logged in users data
///
/// Returns one of the following HttpResponses:
/// * `200` [Body: JSON] - Credentials could be verified
/// * `401` - Missing or invalid JWT
/// * `500` - Something went wrong internally (debug)
///
/// # Arguments
///
/// * `req` - The HttpRequest that was made
/// * `db` - The AppData containing a Mutex-secured Database-connection
///
/// # Examples
///
/// ```text
/// GET-Request at `{api-url}/user` with a cookie containing a valid JWT
/// => 200 [cookie with JWT is set]
///     {
///         "username": "testUser"
///     }
/// ```
/// ```text
/// GET-Request at `{api-url}/user` without a cookie containing a JWT
/// => 401
///     "token-cookie was not found"
/// ```
#[get("/user")]
pub async fn get_user(req: HttpRequest, db: Data<Mutex<Database>>) -> impl Responder {
    match get_user_from_request(req, &db).await {
        Ok(user) => HttpResponse::Ok().json(UserResponse { username: user._id }),
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