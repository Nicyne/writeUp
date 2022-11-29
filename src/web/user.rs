//! Endpoints regarding user-objects and their manipulation

use std::env;
use actix_web::{get, delete, post, Responder, HttpRequest, HttpResponse, web};
use actix_web::web::Data;
use mongodb::bson::doc;
use crate::AppData;
use crate::storage::error::DBError;
use crate::web::auth::{gen_logout_response, get_user_from_request, get_user_id_from_request};
use crate::web::error::APIError;
use crate::web::ResponseObjectWithPayload;
use crate::web::user::json_objects::{UserRequest, UserResponse};

// Response-/Request-Objects
/// Structs modelling the request- and response-bodies
mod json_objects {
    use serde::{Serialize, Deserialize};
    use crate::storage::interface::UserManager;

    /// Body of a response containing user-information
    #[derive(Serialize)]
    pub struct UserResponse {
        /// The username
        pub username: String,
        /// All connected user
        pub relations: Vec<String>
    }

    impl UserResponse {
        pub async fn from(user_manager: &Box<dyn UserManager>) -> UserResponse {
            UserResponse {
                username: user_manager.get_meta_information().name.clone(),
                relations: user_manager.get_associates().await.unwrap() //TODO Catch error
            }
        }
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
/// * `appdata` - An [`AppData`]-instance
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
pub async fn add_user(req: HttpRequest, user_req: web::Json<UserRequest>, appdata: Data<AppData>) -> impl Responder {
    // Check if still logged in
    if get_user_id_from_request(req).is_ok() { //TODO? necessary to be logged out?
        return APIError::NoPermissionError.gen_response()
    }
    // Verify access to beta-deploy
    if user_req.beta_key != env::var("BETA_KEY").unwrap() {
        return APIError::NoPermissionError.gen_response()
    }

    // Add user
    match appdata.get_manager().add_user(&user_req.username, &user_req.password).await {
        Ok(user_manager) => HttpResponse::Created().json(ResponseObjectWithPayload::new(UserResponse::from(&user_manager).await)),
        Err(DBError::InvalidRequestError(_)) => APIError::InvalidCredentialsError(format!("username '{}' already exists", user_req.username)).gen_response(),
        Err(_) => APIError::QueryError("failed to add user".to_string()).gen_response()
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
/// * `appdata` - An [`AppData`]-instance
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
pub async fn get_user(req: HttpRequest, appdata: Data<AppData>) -> impl Responder {
    match get_user_from_request(req, &appdata.get_manager()).await {
        Ok(user_manager) => HttpResponse::Ok().json(ResponseObjectWithPayload::new(
            UserResponse::from(&user_manager).await)),
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
/// * `appdata` - An [`AppData`]-instance
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
pub async fn remove_user(req: HttpRequest, appdata: Data<AppData>) -> impl Responder { //TODO add security check or something (maybe have a body with the user-information or something, password?)
    let db_manager = appdata.get_manager();
    match get_user_from_request(req, &db_manager).await {
        Ok(user_manager) => {
            match db_manager.remove_user(&user_manager.get_meta_information().id).await {
                Ok(_) => gen_logout_response(),
                Err(_) => APIError::QueryError("failed to remove user".to_string()).gen_response()
            }
        },
        Err(e) => e.gen_response()
    }
}