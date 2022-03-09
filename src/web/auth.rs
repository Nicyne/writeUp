use std::sync::Mutex;
use actix_web::{post, HttpResponse, Responder, web, HttpRequest};
use actix_web::cookie::CookieBuilder;
use actix_web::cookie::time::Duration;
use actix_web::web::Data;
use chrono::Utc;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use mongodb::Database;
use crate::db_access::{Credential, CREDENTIALS, DBError, get_dbo_by_id, User, USER};
use crate::web::error::AuthError;
use serde::{Serialize, Deserialize};

// JWT-Assets
const JWT_SECRET: &[u8] = env!("JWT_SECRET").as_bytes(); //TODO secret is static
const JWT_DURATION_MINUTES: i64 = 60;
const JWT_TOKEN_COOKIE_NAME: &str = "writeup_jwt";

#[derive(Debug, Deserialize, Serialize)]
struct Claims {  // Credits to: https://blog.logrocket.com/jwt-authentication-in-rust/
    sub: String,
    exp: usize,
}

// Response-/Request-Objects
mod json_objects {
    use serde::{Serialize, Deserialize};
    #[derive(Deserialize)]
    pub struct TokenRequest {
        pub username: String,
        pub passwd: String
    }
    #[derive(Serialize)]
    pub struct TokenResponse { //TODO Add expire-timestamp
        pub success: bool
    }
}

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
                            .max_age(Duration::minutes(JWT_DURATION_MINUTES)).http_only(true).finish(); //TODO Secure Cookie
                        let mut response = HttpResponse::Ok().json(json_objects::TokenResponse {success: true});
                        if response.add_cookie(&token_cookie).is_err() {return HttpResponse::InternalServerError().finish()} //Cookie couldn't be parsed
                        response
                    }
                    Err(e) => e.gen_response()
                }
            } else { HttpResponse::Unauthorized().finish() } //wrong password
        }
        Err(DBError::NoDocumentFoundError) => HttpResponse::Unauthorized().finish(), //No user with that username has been found
        Err(_) => HttpResponse::InternalServerError().finish() //Unknown
    }
}

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

fn get_user_id_from_request(req: HttpRequest) -> Result<String, AuthError> {
    match req.cookie(JWT_TOKEN_COOKIE_NAME) {
        Some(cookie) => decode::<Claims>(cookie.value(),
                                         &DecodingKey::from_secret(JWT_SECRET),
                                         &Validation::new(Algorithm::HS512))
            .map(|dec|dec.claims.sub).map_err(|_|AuthError::JWTTokenError),
        None => Err(AuthError::MissingAuthError)
    }
}

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
