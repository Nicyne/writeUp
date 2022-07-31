//! Contains functions and endpoints revolving around authorisation and authentication

use std::env;
use std::sync::Mutex;
use actix_web::{post, get, delete, HttpResponse, Responder, web, HttpRequest};
use actix_web::cookie::{CookieBuilder, SameSite, time::Duration};
use actix_web::web::Data;
use chrono::Utc;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use mongodb::bson::doc;
use mongodb::Database;
use crate::db_access::{Credential, CREDENTIALS, DBError, get_dbo_by_id, User, USER};
use crate::web::{error::APIError, ResponseObject, ResponseObjectWithPayload};
use serde::{Serialize, Deserialize};
use crate::JWT_SECRET_ENV_VAR_KEY;

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
        pub password: String
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
///         "password": "testPass"
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
///         "password": "passTest"
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
pub async fn authenticate(db: Data<Mutex<Database>>, creds: web::Json<json_objects::TokenRequest>) -> impl Responder { //TODO Secure credentials
    // Load Credentials for the supposed user
    match get_dbo_by_id::<Credential>(CREDENTIALS, creds.username.as_str().to_string(), db.get_ref()).await {
        Ok(cred) => {
            // Verify their password
            if cred.verify(creds.password.as_str()) {
                // Generate a JWT
                match gen_jwt(creds.username.as_str()) {
                    Ok(jwt) => {
                        // Create a cookie with the JWT
                        let token_cookie = CookieBuilder::new(JWT_TOKEN_COOKIE_NAME, jwt)
                            .same_site(SameSite::Strict)
                            .max_age(Duration::minutes(JWT_DURATION_MINUTES))
                            .http_only(true).finish(); //TODO Secure Cookie
                        let mut response = HttpResponse::Ok().json(ResponseObject::new());
                        if response.add_cookie(&token_cookie).is_err() {
                            return APIError::InternalServerError("failed to set authentication-cookie".to_string()).gen_response()} //Cookie couldn't be parsed
                        response
                    }
                    Err(e) => e.gen_response()
                }
            } else { APIError::InvalidCredentialsError("wrong credentials".to_string()).gen_response() } //wrong password
        }
        Err(DBError::NoDocumentFoundError) => APIError::InvalidCredentialsError("wrong credentials".to_string()).gen_response(), //No user with that username has been found
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
pub async fn get_auth_status(req: HttpRequest, db: Data<Mutex<Database>>) -> impl Responder {
    match get_user_from_request(req, &db).await {
        Ok(user) => HttpResponse::Ok().json(ResponseObjectWithPayload::new(
            doc! {"username": user._id})),
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
pub async fn get_user_from_request(req: HttpRequest, db: &Mutex<Database>) -> Result<User,APIError> {
    // Verify jwt
    match get_user_id_from_request(req) {
        Ok(user_id) => {
            // Extract the user
            match get_dbo_by_id::<User>(USER, user_id, db).await {
                Ok(user) => Ok(user),
                Err(DBError::NoDocumentFoundError) => Err(APIError::AuthenticationError),
                Err(_) => Err(APIError::QueryError("user could not be retrieved from database".to_string()))
            }
        }
        Err(e) => Err(e)
    }
}
