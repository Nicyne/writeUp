use std::sync::Mutex;
use actix_web::{post, HttpResponse, Responder, web};
use actix_web::web::Data;
use chrono::Utc;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use mongodb::Database;
use crate::db_access::{Credential, CREDENTIALS, DBError, get_dbo_by_id};
use crate::web::error::AuthError;
use serde::{Serialize, Deserialize};

// JWT-Assets
const JWT_SECRET: &[u8] = env!("JWT_SECRET").as_bytes(); //TODO secret is static

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
    pub struct TokenResponse {
        pub token: String
    }
}

#[post("/auth")]
pub async fn authenticate(db: Data<Mutex<Database>>, creds: web::Json<json_objects::TokenRequest>) -> impl Responder { //TODO Secure credentials
    match get_dbo_by_id::<Credential>(CREDENTIALS, creds.username.as_str().to_string(), db.get_ref()).await {
        Ok(cred) => {
            if cred.verify(creds.passwd.as_str()) {
                create_jwt(creds.username.as_str()).map(|jwt| HttpResponse::Ok()
                    .json(json_objects::TokenResponse {token: jwt})).unwrap_or_else(|e| e.gen_response())
            } else { HttpResponse::Unauthorized().finish() } //wrong password
        }
        Err(DBError::NoDocumentFoundError) => HttpResponse::Unauthorized().finish(), //No user with that username has been found
        Err(_) => HttpResponse::InternalServerError().finish() //Unknown
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
        .map_err(|_| AuthError::InternalServerError("jwt-token could not be created".to_string()))
}

pub fn get_user_id_from_jwt(jwt: &str) -> Result<String, AuthError> {
    decode::<Claims>(&jwt, &DecodingKey::from_secret(JWT_SECRET), &Validation::new(Algorithm::HS512))
        .map(|dec|dec.claims.sub).map_err(|_|AuthError::JWTTokenError)
}
