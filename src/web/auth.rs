//! Contains functions and endpoints revolving around authorisation and authentication

use std::sync::Mutex;
use actix_web::{post, delete, HttpResponse, Responder, web, HttpRequest};
use actix_web::cookie::{CookieBuilder, SameSite};
use actix_web::cookie::time::Duration;
use actix_web::web::Data;
use chrono::Utc;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use mongodb::Database;
use crate::db_access::{Credential, CREDENTIALS, DBError, get_dbo_by_id, User, USER};
use crate::web::error::AuthError;
use serde::{Serialize, Deserialize};

// JWT-Assets
/// Secret-phrase used to en- and decode JWTs
const JWT_SECRET: &[u8] = env!("JWT_SECRET").as_bytes(); //TODO secret is static
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
    use serde::{Serialize, Deserialize};

    /// Body of an authentication-request
    #[derive(Deserialize)]
    pub struct TokenRequest {
        /// Username
        pub username: String,
        /// Password
        pub passwd: String
    }

    /// Body of a successful authentication-request
    #[derive(Serialize)]
    pub struct TokenResponse { //TODO Add expire-timestamp
        /// Indicator if the authentication worked
        pub success: bool
    }
}

/// ENDPOINT: Takes a set of credentials, verifies them and sets a JWT-cookie as proof
///
/// Returns one of the following HttpResponses:
/// * `200` [COOKIE: JWT] - Credentials could be verified
/// * `401` - Wrong Credentials
/// * `500` - Something went wrong internally (debug)
///
/// # Arguments
///
/// * `db` - The AppData containing a Mutex-secured Database-connection
/// * `creds` - From JSON generated TokenRequest including the credentials to be checked
///
/// # Examples
///
/// ```text
/// POST-Request at `{api-url}/auth`
///     {
///         "username": "testUser",
///         "passwd": "testPass"
///     }
/// => 200 [cookie with JWT is set]
///     {
///         "success": true
///     }
/// ```
/// ```text
/// POST-Request at `{api-url}/auth`
///     {
///         "username": "testUser",
///         "passwd": "passTest"
///     }
/// => 401
///     "wrong credentials"
/// ```
#[post("/auth")]
pub async fn authenticate(db: Data<Mutex<Database>>, creds: web::Json<json_objects::TokenRequest>) -> impl Responder { //TODO Secure credentials
    // Load Credentials for the supposed user
    match get_dbo_by_id::<Credential>(CREDENTIALS, creds.username.as_str().to_string(), db.get_ref()).await {
        Ok(cred) => {
            // Verify their password
            if cred.verify(creds.passwd.as_str()) {
                // Generate a JWT
                match gen_jwt(creds.username.as_str()) {
                    Ok(jwt) => {
                        // Create a cookie with the JWT
                        let token_cookie = CookieBuilder::new(JWT_TOKEN_COOKIE_NAME, jwt)
                            .same_site(SameSite::Strict)
                            .max_age(Duration::minutes(JWT_DURATION_MINUTES))
                            .http_only(true).finish(); //TODO Secure Cookie
                        let mut response = HttpResponse::Ok().json(json_objects::TokenResponse {success: true});
                        if response.add_cookie(&token_cookie).is_err() {return AuthError::InternalServerError("failed to set cookie".to_string()).gen_response()} //Cookie couldn't be parsed
                        response
                    }
                    Err(e) => e.gen_response()
                }
            } else { AuthError::WrongCredentialsError.gen_response() } //wrong password
        }
        Err(DBError::NoDocumentFoundError) => AuthError::InvalidUserError.gen_response(), //No user with that username has been found
        Err(_) => AuthError::InternalServerError("failed to access credentials".to_string()).gen_response() //Unknown
    }
}

/// ENDPOINT: Removes all verification from the client to effectively log them out
///
/// Returns one of the following HttpResponses:
/// * `200` [REMOVAL_COOKIE: JWT] - Valid JWT-cookie found
/// * `401` - No or invalid JWT-cookie found
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
/// ```
/// ```text
/// DELETE-Request at `{api-url}/auth` without a cookie containing a JWT
/// => 401
///     "token-cookie was not found"
/// ```
/// ```text
/// DELETE-Request at `{api-url}/auth` with a cookie containing an invalid JWT
/// => 401
///     "jwt token not valid"
/// ```
#[allow(unused_must_use)]
#[delete("/auth")]
pub async fn logout(req: HttpRequest) -> impl Responder {
    match get_user_id_from_request(req) {
        Ok(_) => {
            let mut resp = HttpResponse::Ok().finish();
            resp.add_removal_cookie(&CookieBuilder::new(JWT_TOKEN_COOKIE_NAME, "-1").same_site(SameSite::Strict).http_only(true).finish());
            resp
        }
        Err(e) => e.gen_response()
    }
}

/// Creates a JWT with the username as its payload
///
/// # Arguments
///
/// * `uid` - The username to save
fn gen_jwt(uid: &str) -> Result<String, AuthError> {
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
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| AuthError::InternalServerError("jwt-token could not be created".to_string()))
}

/// Retrieves the username of the current user using the JWT-cookie in a HttpRequest
///
/// # Arguments
///
/// * `req` - HttpRequest from which the cookie and therefore the JWT gets extracted
pub fn get_user_id_from_request(req: HttpRequest) -> Result<String, AuthError> { //TODO Make private
    match req.cookie(JWT_TOKEN_COOKIE_NAME) {
        Some(cookie) => decode::<Claims>(cookie.value(),
                                         &DecodingKey::from_secret(JWT_SECRET),
                                         &Validation::new(Algorithm::HS512))
            .map(|dec|dec.claims.sub).map_err(|_|AuthError::JWTTokenError),
        None => Err(AuthError::MissingAuthError)
    }
}


/// Retrieves a User-object from the DB using the JWT-cookie in a HttpRequest
///
/// # Arguments
///
/// * `req` - HttpRequest from which the cookie and therefore the JWT gets extracted
/// * `db` - Reference to a Mutex-secured Database-connection
pub async fn get_user_from_request(req: HttpRequest, db: &Mutex<Database>) -> Result<User,AuthError> {
    // Verify jwt
    match get_user_id_from_request(req) {
        Ok(user_id) => {
            // Extract the user
            match get_dbo_by_id::<User>(USER, user_id, db).await {
                Ok(user) => Ok(user),
                Err(DBError::NoDocumentFoundError) => Err(AuthError::InvalidUserError),
                Err(_) => Err(AuthError::InternalServerError("could not retrieve user from database".to_string()))
            }
        }
        Err(e) => Err(e)
    }
}
