use actix_web::HttpResponse;
use thiserror::Error;
use serde::Serialize;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("wrong credentials")]
    WrongCredentialsError,
    #[error("jwt token not valid")]
    JWTTokenError,
    #[error("jwt token creation error")]
    JWTTokenCreationError,
    #[error("no permission")]
    NoPermissionError,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String
}

impl AuthError {
    pub fn gen_response(&self) -> HttpResponse {
        match self {
            AuthError::WrongCredentialsError => HttpResponse::Forbidden().json(self.get_response_struct()),
            AuthError::JWTTokenError => HttpResponse::Unauthorized().json(self.get_response_struct()),
            AuthError::JWTTokenCreationError => HttpResponse::InternalServerError().json(self.get_response_struct()),
            AuthError::NoPermissionError => HttpResponse::Unauthorized().json(self.get_response_struct())
        }
    }

    fn get_response_struct(&self) -> ErrorResponse {
        ErrorResponse {
            error: self.to_string()
        }
    }
}