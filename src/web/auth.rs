//! Contains functions and endpoints revolving around authorisation and authentication

use std::env;
use actix_web::{post, get, delete, HttpResponse, Responder, web, HttpRequest};
use actix_web::cookie::{CookieBuilder, SameSite, time::Duration};
use actix_web::web::Data;
use chrono::Utc;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use mongodb::bson::doc;
use crate::web::{error::APIError, ResponseObject, ResponseObjectWithPayload};
use serde::{Serialize, Deserialize};
use crate::{AppData, has_dev_flag, JWT_SECRET_ENV_VAR_KEY};
use crate::storage::error::DBError;
use crate::storage::interface::{DBManager, UserManager};

// JWT-Assets
/// Time in minutes until a JWT expires
const JWT_DURATION_MINUTES: i64 = 60;
/// Name of the cookie carrying the JWT
const JWT_TOKEN_COOKIE_NAME: &str = "writeup_jwt";

/// Struct containing all information to be encoded in the JWT
#[derive(Debug, Deserialize, Serialize)]
struct Claims {  // Credits to: https://blog.logrocket.com/jwt-authentication-in-rust/
    /// Username of the user to be authorised
    sub: String,
    /// Timestamp of JWT-expiration
    exp: usize,
}

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
/// * `db` - The AppData containing a Mutex-secured Database-connection
/// * `creds` - From JSON generated TokenRequest including the credentials to be checked
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
pub async fn authenticate(db_pool: Data<AppData>, creds: web::Json<json_objects::TokenRequest>) -> impl Responder {
    let db = db_pool.get_manager();
    // Verify their password
    match db.auth_user(&creds.username, &creds.password).await {
        Ok(_user_manager) => {
            // Generate a JWT
            match gen_jwt(creds.username.as_str()) {
                Ok(jwt) => {
                    // Create a cookie with the JWT
                    let token_cookie_builder = CookieBuilder::new(JWT_TOKEN_COOKIE_NAME, jwt)
                        .same_site(SameSite::Strict)
                        .http_only(true)
                        .secure(!has_dev_flag());
                    let token_cookie = if !creds.session_only {
                        token_cookie_builder.max_age(Duration::minutes(JWT_DURATION_MINUTES)).finish()
                    } else {
                        token_cookie_builder.finish()
                    };
                    let mut response = HttpResponse::Ok().json(ResponseObject::new());
                    if response.add_cookie(&token_cookie).is_err() {
                        return APIError::InternalServerError("failed to set authentication-cookie".to_string()).gen_response()} //Cookie couldn't be parsed
                    response
                }
                Err(e) => e.gen_response()
            }
        },
        Err(DBError::IncorrectCredentialsError) => APIError::InvalidCredentialsError("wrong credentials".to_string()).gen_response(), //could not verify a user with these credentials
        Err(_) => APIError::QueryError("can not access credentials".to_string()).gen_response() //Unknown
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
/// * `req` - The HttpRequest that was made
/// * `db` - The AppData containing a Mutex-secured Database-connection
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
pub async fn get_auth_status(req: HttpRequest, db_pool: Data<AppData>) -> impl Responder {
    match get_user_from_request(req, &db_pool.get_manager()).await {
        Ok(user_manager) => HttpResponse::Ok().json(ResponseObjectWithPayload::new(
            doc! { "username": user_manager.get_meta_information().id.clone() })),
        Err(APIError::AuthenticationError) => {
            let mut response = ResponseObject::new();
            response.success = false;
            HttpResponse::Ok().json(response)
        }
        Err(e) => e.gen_response()
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
/// * `req` - The HttpRequest that was made
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
pub async fn logout(req: HttpRequest) -> impl Responder {
    match get_user_id_from_request(req) {
        Ok(_) => gen_logout_response(),
        Err(e) => e.gen_response()
    }
}

#[allow(unused_must_use)]
/// Creates a Response that revokes any form of login-verification
pub fn gen_logout_response() -> HttpResponse {
    let mut resp = HttpResponse::Ok().json(ResponseObject::new());
    resp.add_removal_cookie(&CookieBuilder::new(JWT_TOKEN_COOKIE_NAME, "-1")
        .same_site(SameSite::Strict).http_only(true).finish());
    resp
}

/// Creates a JWT with the username as its payload
///
/// # Arguments
///
/// * `uid` - The username to save
fn gen_jwt(uid: &str) -> Result<String, APIError> {
    // Set all required values
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::minutes(JWT_DURATION_MINUTES))
        .expect("Not a valid timestamp")
        .timestamp();
    let claims = Claims {
        sub: uid.to_owned(),
        exp: expiration as usize
    };
    let header = Header::new(Algorithm::HS512);

    // Generate the JWT
    encode(&header, &claims, &EncodingKey::from_secret(env::var(JWT_SECRET_ENV_VAR_KEY).unwrap().as_bytes()))
        .map_err(|_| APIError::InternalServerError("jwt-token creation failed".to_string()))
}

/// Retrieves the username of the current user using the JWT-cookie in a HttpRequest
///
/// # Arguments
///
/// * `req` - HttpRequest from which the cookie and therefore the JWT gets extracted
pub fn get_user_id_from_request(req: HttpRequest) -> Result<String, APIError> { //TODO Make private
    match req.cookie(JWT_TOKEN_COOKIE_NAME) {
        Some(cookie) => decode::<Claims>(cookie.value(),
                                         &DecodingKey::from_secret(env::var(JWT_SECRET_ENV_VAR_KEY).unwrap().as_bytes()),
                                         &Validation::new(Algorithm::HS512))
            .map(|dec|dec.claims.sub).map_err(|_|APIError::AuthenticationError), // Invalid JWT
        None => Err(APIError::AuthenticationError) // No JWT-cookie
    }
}


/// Retrieves a User-object from the DB using the JWT-cookie in a HttpRequest
///
/// # Arguments
///
/// * `req` - HttpRequest from which the cookie and therefore the JWT gets extracted
/// * `db` - Reference to a Mutex-secured Database-connection
pub async fn get_user_from_request(req: HttpRequest, db: &Box<dyn DBManager>) -> Result<Box<dyn UserManager>,APIError> {
    // Verify jwt
    match get_user_id_from_request(req) {
        Ok(user_id) => {
            // Extract the user
            match db.get_user(&user_id).await {
                Ok(user) => Ok(user),
                Err(DBError::MissingEntryError(_,_)) => Err(APIError::AuthenticationError),
                Err(DBError::InvalidSequenceError(_)) => Err(APIError::InvalidIDError),
                Err(_) => Err(APIError::QueryError("user could not be retrieved from database".to_string()))
            }
        }
        Err(e) => Err(e)
    }
}
