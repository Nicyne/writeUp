//! Contains functions and endpoints revolving around authorisation and authentication

use std::future::Future;
use std::pin::Pin;
use actix_identity::Identity;
use actix_web::{post, get, delete, HttpResponse, Responder, web, HttpRequest, HttpMessage, FromRequest};
use actix_web::dev::Payload;
use actix_web::web::Data;
use mongodb::bson::doc;
use crate::web::{error::APIError, ResponseObject, ResponseObjectWithPayload};
use crate::AppData;
use crate::storage::error::DBError;
use crate::storage::interface::UserManager;

// Response-/Request-Objects
/// Structs modelling the request- and response-bodies
mod json_objects {
    use serde::Deserialize;

    /// Body of an authentication-request
    #[derive(Deserialize)]
    pub struct TokenRequest {
        /// Username
        pub username: String,
        /// Password
        pub password: String,
        /// Define whether or not to verify the user for longer than a single session
        pub session_only: bool
    }
}


// Extractor for UserManager to automatically check if the user is authenticated
impl FromRequest for Box<dyn UserManager> {
    type Error = APIError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let appdata = req.app_data::<Data<AppData>>().unwrap().to_owned();
        let identity_extractor = Identity::extract(req);

        Box::pin(async move {
            match identity_extractor.await {
                Ok(identity) => {
                    let db_manager = appdata.get_manager();
                    match db_manager.get_user(&identity.id().unwrap()).await {
                        Ok(user_manager) => Ok(user_manager),
                        Err(DBError::MissingEntryError(_,_)) => Err(APIError::AuthenticationError),
                        Err(DBError::InvalidSequenceError(_)) => Err(APIError::InvalidIDError),
                        Err(_) => Err(APIError::QueryError("user could not be retrieved from database".to_string()))
                    }
                }
                Err(_) => Err(APIError::AuthenticationError)
            }
        })
    }
}


/// ENDPOINT: Takes a set of credentials, verifies them and sets a JWT-cookie as proof
///
/// Returns one of the following HttpResponses:
/// * `200`
///     - \[COOKIE: JWT\] Credentials could be verified
///     - **\[11\]** Credentials are incorrect
/// * `500`
///     - Something went wrong internally (debug)
///
/// # Arguments
///
/// * `req` - The HttpRequest that was made
/// * `creds` - From JSON generated TokenRequest including the credentials to be checked
/// * `appdata` - An [`AppData`]-instance
///
/// # Examples
///
/// ```text
/// POST-Request at `{api-url}/auth` (valid credentials)
///     {
///         "username": "testUser",
///         "password": "testPass",
///         "session_only": false
///     }
/// => 200 [cookie with JWT is set]
///     {
///         "success": true,
///         "time": "2022-04-11 12:05:57"
///     }
/// ```
/// ```text
/// POST-Request at `{api-url}/auth` (invalid credentials)
///     {
///         "username": "testUser",
///         "password": "passTest",
///         "session_only": false
///     }
/// => 200
///     {
///         "success": false,
///         "code": 11,
///         "message": "failed to process credentials: wrong credentials",
///         "time": "2022-04-11 12:05:57"
///     }
/// ```
#[post("/auth")]
pub async fn authenticate(req: HttpRequest, creds: web::Json<json_objects::TokenRequest>, appdata: Data<AppData>) -> impl Responder {
    let db = appdata.get_manager();
    // Verify their password
    match db.auth_user(&creds.username, &creds.password).await {
        Ok(user_manager) => {
            // Initiate a session
            Identity::login(&req.extensions(),
                            user_manager.get_meta_information().id.clone())
                .unwrap();
            Ok(HttpResponse::Ok().json(ResponseObject::new()))
        },
        Err(DBError::IncorrectCredentialsError) =>
            Err(APIError::InvalidCredentialsError("wrong credentials".to_string())), //could not verify a user with these credentials
        Err(_) => Err(APIError::QueryError("can not access credentials".to_string())) //Unknown
    }
}

/// ENDPOINT: Checks if a user is currently logged in
///
/// Returns one of the following HttpResponses:
/// * `200`
///     - Valid JWT-cookie was found
///     - No valid JWT-cookie was found
/// * `500`
///     - Something went wrong internally (debug)
///
/// # Arguments
///
/// * `user` - An Option containing the users identity if known
///
/// # Examples
///
/// ```text
/// GET-Request at `{api-url}/auth` with a cookie containing a valid JWT
/// => 200
///     {
///         "success": true,
///         "content": {
///             "username": "testUser"
///         },
///         "time": "2022-04-11 12:05:57"
///     }
/// ```
/// ```text
/// GET-Request at `{api-url}/auth` without a valid cookie containing a JWT
/// => 200
///     {
///         "success": false,
///         "time": "2022-04-11 12:05:57"
///     }
/// ```
#[get("/auth")]
pub async fn get_auth_status(user: Option<Identity>) -> impl Responder {
    match user {
        Some(ident) => HttpResponse::Ok().json(
            ResponseObjectWithPayload::new(
                doc! { "username": ident.id().unwrap() }
            )),
        None => {
            let mut response = ResponseObject::new();
            response.success = false;
            HttpResponse::Ok().json(response)
        }
    }
}

/// ENDPOINT: Removes all verification from the client to effectively log them out
///
/// Returns one of the following HttpResponses:
/// * `200`
///     - \[REMOVAL_COOKIE: JWT\] Valid JWT-cookie found
/// * `401`
///     - **\[10\]** No or invalid JWT-cookie found
///
/// # Arguments
///
/// * `user` - The users identity
///
/// # Examples
///
/// ```text
/// DELETE-Request at `{api-url}/auth` with a cookie containing a valid JWT
/// => 200 [cookie is removed]
///     {
///         "success": true,
///         "time": "2022-04-11 12:05:57"
///     }
/// ```
/// ```text
/// DELETE-Request at `{api-url}/auth` without a valid cookie containing a JWT
/// => 401
///     {
///         "success": false,
///         "code": 10,
///         "message": "user is not logged in",
///         "time": "2022-04-11 12:05:57"
///     }
/// ```
#[delete("/auth")]
pub async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::Ok().json(ResponseObject::new())
}
