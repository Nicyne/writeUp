//! Endpoints regarding user-objects and their manipulation

use std::env;
use std::sync::Mutex;
use actix_web::{get, delete, post, Responder, HttpRequest, HttpResponse, web};
use actix_web::web::Data;
use mongodb::bson::doc;
use mongodb::Database;
use crate::db_access::{Credential, CREDENTIALS, del_dbo_by_id, get_dbo_by_id, insert_dbo, Note, NOTES, update_dbo_by_id, User, USER};
use crate::db_access::AllowanceLevel::Owner;
use crate::db_access::DBError::{NoDocumentFoundError, QueryError};
use crate::web::auth::{gen_logout_response, get_user_from_request, get_user_id_from_request};
use crate::web::error::APIError;
use crate::web::ResponseObjectWithPayload;
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

    /// Body of a request for a new user
    #[derive(Deserialize)]
    pub struct UserRequest {
        /// The identifier of the new user
        pub username: String,
        /// The password associated with the new user
        pub password: String,
        /// A key indicating the user has access to the beta-deployment
        pub beta_key: String
    }
}

/// ENDPOINT: Creates a new user with the given credentials
///
/// Returns one of the following HttpResponses:
/// * `201`
///     - \[Body: JSON\] User was successfully created
/// * `200`
///     - **\[11\]** Username already exists in the database
/// * `403`
///     - **\[12\]** User is currently logged in
/// * `500`
///     - Something went wrong internally (debug)
///
/// # Arguments
///
/// * `req` - The HttpRequest that was made
/// * `user_req` - The body of the request parsed to a UserRequest-object
/// * `db` - The AppData containing a Mutex-secured Database-connection
///
/// # Examples
///
/// ```text
/// POST-Request at `{api-url}/user` with no token-cookie
///     {
///         "username": "otherUser",
///         "password": "otherPass",
///         "beta_key": "B757B"
///     }
/// => 201
///     {
///         "success": true,
///         "content": {
///             "username": "otherUser",
///             "relations": []
///         },
///         "time": "2022-04-11 12:20:28"
///     }
/// ```
/// ```text
/// POST-Request at `{api-url}/user` with no token-cookie (username already exists in db)
///     {
///         "username": "testUser",
///         "password": "otherPass",
///         "beta_key": "B757B"
///     }
/// => 200
///     {
///         "success": false,
///         "code": 11,
///         "message": "failed to process credentials: username already exists",
///         "time": "2022-04-11 12:20:19"
///     }
/// ```
/// ```text
/// POST-Request at `{api-url}/user` with a valid token-cookie
///     {
///         "username": "otherUser",
///         "password": "otherPass",
///         "beta_key": "B757B"
///     }
/// => 403
///     {
///         "success": false,
///         "code": 12,
///         "message": "no permission",
///         "time": "2022-04-11 12:20:19"
///     }
/// ```
/// ```text
/// POST-Request at `{api-url}/user` with an invalid beta-key
///     {
///         "username": "otherUser",
///         "password": "otherPass",
///         "beta_key": "B757C"
///     }
/// => 403
///     {
///         "success": false,
///         "code": 12,
///         "message": "no permission",
///         "time": "2022-04-11 12:20:19"
///     }
/// ```
#[post("/user")]
pub async fn add_user(req: HttpRequest, user_req: web::Json<UserRequest>, db: Data<Mutex<Database>>) -> impl Responder {
    // Check if still logged in
    if get_user_id_from_request(req).is_ok() { //TODO? necessary to be logged out?
        return APIError::NoPermissionError.gen_response()
    }
    // Verify access to beta-deploy
    if user_req.beta_key != env::var("BETA_KEY").unwrap() {
        return APIError::NoPermissionError.gen_response()
    }
    let new_user = user_req.into_inner();
    // Check for unique username
    match get_dbo_by_id::<Credential>(CREDENTIALS, new_user.username.clone(), &db).await {
        Err(NoDocumentFoundError) => {
            // Prepare the new dbos
            let creds = Credential::new(new_user.username.clone(), &new_user.password);
            let user = User {_id: new_user.username, allowances: Vec::new(), connections: Vec::new()};

            // Insert the new dbos
            let add_cred = insert_dbo::<Credential>(CREDENTIALS, &creds, &db);
            let add_user = insert_dbo::<User>(USER, &user, &db);

            // Check for error
            if add_cred.await.is_err() || add_user.await.is_err() {
                return APIError::QueryError("user/credentials could not be created".to_string()).gen_response()
            }
            HttpResponse::Created().json(ResponseObjectWithPayload::new(UserResponse {username: user._id.clone(), relations: user.connections.clone()})) //TODO? login afterwards?
        }
        Ok(_) => APIError::InvalidCredentialsError("username already exists".to_string()).gen_response(),
        Err(_) => APIError::QueryError("username could not be checked on uniqueness".to_string()).gen_response()
    }
}

/// ENDPOINT: Returns the currently logged in users data
///
/// Returns one of the following HttpResponses:
/// * `200`
///     - \[Body: JSON\] Credentials could be verified
/// * `401`
///     - **\[10\]** Missing or invalid JWT
/// * `500`
///     - Something went wrong internally (debug)
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
///         "success": true,
///         "content": {
///             "username": "testUser",
///             "relations": ["otherUser", "yetAnotherUser"]
///         },
///         "time": "2022-04-11 12:20:28"
///     }
/// ```
/// ```text
/// GET-Request at `{api-url}/user` without a cookie containing a JWT
/// => 401
///     {
///         "success": false,
///         "code": 10,
///         "message": "user is not logged in",
///         "time": "2022-04-11 12:20:19"
///     }
/// ```
#[get("/user")]
pub async fn get_user(req: HttpRequest, db: Data<Mutex<Database>>) -> impl Responder {
    match get_user_from_request(req, &db).await {
        Ok(user) => HttpResponse::Ok().json(ResponseObjectWithPayload::new(
            UserResponse { username: user._id, relations: user.connections })),
        Err(e) => e.gen_response()
    }
}

/// ENDPOINT: Removes a user from the database and logs them out
///
/// Returns one of the following HttpResponses:
/// * `200`
///     - User was removed successfully
/// * `401`
///     - Missing or invalid JWT
/// * `500`
///     - Something went wrong internally (debug)
///
/// # Arguments
///
/// * `req` - The HttpRequest that was made
/// * `db` - The AppData containing a Mutex-secured Database-connection
///
/// # Examples
///
/// ```text
/// DELETE-Request at `{api-url}/user` with a cookie containing a valid JWT
/// => 200 [cookie is removed]
///     {
///         "success": true,
///         "time": "2022-04-11 12:05:57"
///     }
/// ```
/// ```text
/// DELETE-Request at `{api-url}/user` without a cookie containing a JWT
/// => 401
///     {
///         "success": false,
///         "code": 10,
///         "message": "user is not logged in",
///         "time": "2022-04-11 12:20:19"
///     }
/// ```
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
                return APIError::QueryError("notes and allowances could not be fully removed".to_string()).gen_response()
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
                return APIError::QueryError("relations to other user could not be fully removed".to_string()).gen_response()
            }

            // Remove the user and his credentials
            let user_removal = del_dbo_by_id::<User>(USER,
                                                     user._id.clone(), &db);
            let cred_removal = del_dbo_by_id::<Credential>(CREDENTIALS,
                                                           user._id.clone(), &db);
            if user_removal.await.is_err() || cred_removal.await.is_err() {
                return APIError::QueryError("user and/or credentials could not be removed".to_string()).gen_response()
            }

            // Log the user out
            gen_logout_response()
        }
        Err(e) => e.gen_response()
    }
}