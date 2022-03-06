//! REST-Endpoints and their logic

mod note;
mod user;
mod share;
mod error;

use serde::{Serialize, Deserialize};
use actix_web::{get, HttpResponse, Responder, web};
use actix_web_httpauth::extractors::{basic::BasicAuth, bearer::BearerAuth};
use chrono::Utc;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use crate::web::error::AuthError;

const JWT_SECRET: &[u8] = env!("JWT_SECRET").as_bytes(); //TODO secret is static

#[derive(Debug, Deserialize, Serialize)]
struct Claims {  // Credits to: https://blog.logrocket.com/jwt-authentication-in-rust/
    sub: String,
    exp: usize,
}

#[derive(Serialize)]
struct TokenResponse {
    token: String
}

pub fn handler_config(cfg: &mut web::ServiceConfig) {
    // Add all special handler
    cfg.service(authenticate)
        .service(list_notes);
    // Add all note-related handler
    cfg.service(note::add_note)
        .service(note::get_note)
        .service(note::update_note)
        .service(note::remove_note);
    // Add all user-related handler
    cfg.service(user::add_user)
        .service(user::get_user)
        .service(user::update_user)
        .service(user::remove_user);
    // Add all share-related handler
    cfg.service(share::create_relation)
        .service(share::get_relation_code)
        .service(share::update_share);
}

#[get("/auth")]
async fn authenticate(auth: BasicAuth) -> impl Responder {
    //TODO Verify credentials
    match create_jwt(auth.user_id()) {
        Ok(jwt) => HttpResponse::Ok().json(TokenResponse {token: jwt}),
        Err(e) => e.gen_response()
    }
}

#[get("/notes")]
async fn list_notes(auth: BearerAuth) -> impl Responder { //TODO implement
    let user = get_user_from_jwt(auth.token());
    match user {
        Ok(user_id) => HttpResponse::Ok().json(format!("Request for listing all owned and shared notes of '{}' received", user_id)),
        Err(e) => e.gen_response()
    }
}

fn create_jwt(uid: &str) -> Result<String, AuthError> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("Not a valid timestamp")
        .timestamp();
    let claims = Claims {
        sub: uid.to_owned(),
        exp: expiration as usize
    };
    let header = Header::new(Algorithm::HS512);

    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| AuthError::JWTTokenCreationError)
}

fn get_user_from_jwt(jwt: &str) -> Result<String, AuthError> {
    match decode::<Claims>(&jwt, &DecodingKey::from_secret(JWT_SECRET),
                           &Validation::new(Algorithm::HS512)) {
        Ok(decoded) => Ok(decoded.claims.sub),
        Err(_) => Err(AuthError::JWTTokenError)
    }
}
