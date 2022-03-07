//! REST-Endpoints and their logic

mod note;
mod user;
mod share;
mod error;

use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use actix_web::{get, HttpResponse, Responder, web::{ServiceConfig, Data}};
use actix_web_httpauth::extractors::{basic::BasicAuth, bearer::BearerAuth};
use chrono::Utc;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use mongodb::{bson::doc, Database};
use crate::db_access::{AllowanceLevel, Credential, CREDENTIALS, DBError, get_dbo_by_id, Note, NOTES, User, USER};
use crate::web::error::AuthError;

// JWT-Assets
const JWT_SECRET: &[u8] = env!("JWT_SECRET").as_bytes(); //TODO secret is static

#[derive(Debug, Deserialize, Serialize)]
struct Claims {  // Credits to: https://blog.logrocket.com/jwt-authentication-in-rust/
    sub: String,
    exp: usize,
}

// Response-Objects
#[derive(Serialize)]
struct NoteResponse {
    note: Note,
    allowance: AllowanceLevel
}

pub fn handler_config(cfg: &mut ServiceConfig) {
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
async fn authenticate(auth: BasicAuth, db: Data<Mutex<Database>>) -> impl Responder {
    #[derive(Serialize)]
    struct TokenResponse {
        token: String
    }
    match get_dbo_by_id::<Credential>(CREDENTIALS, auth.user_id().to_string(), db.get_ref()).await {
        Ok(cred) => {
            if cred.verify(auth.password().unwrap()) {
                create_jwt(auth.user_id()).map(|jwt| HttpResponse::Ok()
                    .json(TokenResponse {token: jwt})).unwrap_or_else(|e| e.gen_response())
            } else { HttpResponse::Forbidden().finish() } //wrong password
        }
        Err(DBError::NoDocumentFoundError) => HttpResponse::Forbidden().finish(), //No user with that username has been found
        Err(_) => HttpResponse::InternalServerError().finish() //Unknown
    }
}

#[get("/notes")]
async fn list_notes(auth: BearerAuth, db: Data<Mutex<Database>>) -> impl Responder {
    // Define Response-Object
    #[derive(Serialize)]
    struct ReducedNoteResponse {
        note_id: String,
        title: String,
        tags: Vec<String>,
        allowance: AllowanceLevel
    }
    // Verify Credentials
    let user = get_user_from_jwt(auth.token());
    match user {
        Ok(user_id) => {
            // Get list of allowed notes
            match get_dbo_by_id::<User>(USER, user_id, db.get_ref()).await {
                Ok(user) => {
                    let mut response_vector = Vec::new();
                    for allowance in user.allowances {
                        // Read all allowed notes and create response-objects
                        match get_dbo_by_id::<Note>(NOTES, allowance.note_id.clone(), db.get_ref()).await {
                            Ok(note) => response_vector.push(ReducedNoteResponse {
                                note_id: allowance.note_id,
                                title: note.title,
                                tags: note.tags,
                                allowance: allowance.level
                            }),
                            Err(DBError::NoDocumentFoundError) => return HttpResponse::InternalServerError()
                                .json(format!("Reference to deleted note(ID:{}) still exists", allowance.note_id)), //user has allowance for a nonexisting note)
                            Err(_) => {} //unknown
                        }
                    }
                    // Return the compiled list of notes
                    HttpResponse::Ok().json(response_vector)
                }
                Err(DBError::NoDocumentFoundError) => HttpResponse::InternalServerError()
                    .json("User-Object is missing"), //there are credentials for a nonexisting user
                Err(_) => HttpResponse::InternalServerError().finish() //unknown
            }
        }
        Err(e) => e.gen_response()
    }
}

fn create_jwt(uid: &str) -> Result<String, AuthError> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::minutes(60))
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
    decode::<Claims>(&jwt, &DecodingKey::from_secret(JWT_SECRET), &Validation::new(Algorithm::HS512))
        .map(|dec|dec.claims.sub).map_err(|_|AuthError::JWTTokenError)
}
