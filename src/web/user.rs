//! Endpoints regarding user-objects and their manipulation

use std::sync::Mutex;
use actix_web::{get, put, delete, post, Responder, HttpRequest, HttpResponse, web};
use actix_web::cookie::{CookieBuilder, SameSite};
use actix_web::web::{Data, Path};
use mongodb::bson::doc;
use mongodb::Database;
use crate::db_access::{Credential, CREDENTIALS, del_dbo_by_id, get_dbo_by_id, insert_dbo, is_safe, Note, NOTES, update_dbo_by_id, User, USER};
use crate::db_access::AllowanceLevel::Owner;
use crate::db_access::DBError::{NoDocumentFoundError, QueryError};
use crate::web::auth::{get_user_from_request, get_user_id_from_request, JWT_TOKEN_COOKIE_NAME};
use crate::web::error::AuthError;
use crate::web::error::AuthError::InternalServerError;
use crate::web::user::json_objects::{UserRequest, UserResponse};

// Response-/Request-Objects
/// Structs modelling the request- and response-bodies
mod json_objects {
    use serde::{Serialize, Deserialize};

    /// Body of a response containing user-information
    #[derive(Serialize)]
    pub struct UserResponse {
        /// The username
        pub username: String,
        /// All connected user
        pub relations: Vec<String>
    }

    #[derive(Deserialize)]
    pub struct UserRequest {
        pub username: String,
        pub passwd: String //TODO DO NOT USE PLAIN TEXT PASSWORDS
    }
}

#[post("/user")]
pub async fn add_user(req: HttpRequest, user_req: web::Json<UserRequest>, db: Data<Mutex<Database>>) -> impl Responder {
    // Check if still logged in
    if get_user_id_from_request(req).is_ok() { //TODO? necessary to be logged out?
        return InternalServerError("Still logged in".to_string()).gen_response() //TODO Review error-types
    }
    let new_user = user_req.into_inner();
    // Check for unique username
    match get_dbo_by_id::<Credential>(CREDENTIALS, new_user.username.clone(), &db).await {
        Err(NoDocumentFoundError) => {
            // Prepare the new dbos
            let creds = Credential {_id: new_user.username.clone(), passwd: new_user.passwd};
            let user = User {_id: new_user.username, allowances: Vec::new(), connections: Vec::new()};

            // Insert the new dbos
            let add_cred = insert_dbo::<Credential>(CREDENTIALS, &creds, &db);
            let add_user = insert_dbo::<User>(USER, &user, &db);

            // Check for error
            if add_cred.await.is_err() || add_user.await.is_err() {
                return InternalServerError("Could not create user".to_string()).gen_response()
            }
            HttpResponse::Ok().json(UserResponse {username: user._id.clone(), relations: user.connections.clone()}) //TODO? login afterwards?
        }
        Ok(_) => InternalServerError("User already exists".to_string()).gen_response(), //TODO Review error-types
        Err(_) => InternalServerError("Something went wrong when verifying username".to_string()).gen_response() //TODO Review error-types
    }
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
///         "username": "testUser",
///         "relations": ["otherUser", "yetAnotherUser"]
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
        Ok(user) => HttpResponse::Ok().json(UserResponse { username: user._id, relations: user.connections }),
        Err(e) => e.gen_response()
    }
}

#[delete("/user")]
pub async fn remove_user(req: HttpRequest, db: Data<Mutex<Database>>) -> impl Responder { //TODO add security check or something (maybe have a body with the user-information or something, password?)
    match get_user_from_request(req, &db).await { //TODO This function borrows a lot of lines from other endpoints
        Ok(user) => {
            // Remove all notes and their allowances
            let mut note_deletion_error = Vec::new();
            for note in user.allowances { //TODO Multithread
                if note.level == Owner {
                    // Remove all allowances
                    let users = db.lock().unwrap().collection::<User>(USER);
                    match users.update_many(doc! {}, doc! {"$pull": {"allowances": {"note_id": &note.note_id}}}, None).await { //TODO Dont queue through all notes
                        Ok(_res) => {
                            // Remove note
                            if del_dbo_by_id::<Note>(NOTES, note.note_id, &db).await.is_err() {
                                note_deletion_error.push(QueryError) //TODO error-report?
                            }
                        }
                        Err(_) => note_deletion_error.push(QueryError) //TODO error-report?
                    }
                }
            }
            // Check for missed notes/allowances
            if !note_deletion_error.is_empty() {
                return InternalServerError("Could not delete all notes or allowances".to_string()).gen_response() //TODO Review error-types
            }

            // Remove all Relations with other user
            let mut connection_deletion_error = Vec::new();
            for conn_user in user.connections { //TODO Multithread
                if update_dbo_by_id::<User>(USER, conn_user,
                                         doc! {"$pull": {"connections": &user._id}},
                                         &db).await.is_err() {
                    connection_deletion_error.push(QueryError) //TODO error-report?
                }
            }
            // Check for missed relations
            if !connection_deletion_error.is_empty() {
                return InternalServerError("Could not remove all relations to other user".to_string()).gen_response() //TODO Review error-types
            }

            // Remove the user and his credentials
            let user_removal = del_dbo_by_id::<User>(USER,
                                                     user._id.clone(), &db);
            let cred_removal = del_dbo_by_id::<Credential>(CREDENTIALS,
                                                           user._id.clone(), &db);
            if user_removal.await.is_err() || cred_removal.await.is_err() {
                return InternalServerError("Could not delete user or credentials".to_string()).gen_response() //TODO Review error-types
            }

            // Log the user out
            let mut resp = HttpResponse::Ok().json(doc! {"success": true});
            resp.add_removal_cookie(&CookieBuilder::new(JWT_TOKEN_COOKIE_NAME, "-1")
                .same_site(SameSite::Strict).http_only(true).finish());
            resp
        }
        Err(e) => e.gen_response()
    }
}