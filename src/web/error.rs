use actix_web::HttpResponse;
use thiserror::Error;
use serde::Serialize;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("wrong credentials")]
    WrongCredentialsError,
    #[error("jwt token not valid")]
    JWTTokenError,
    #[error("internal server error: {0}")]
    InternalServerError(String),
    #[error("no permission")]
    NoPermissionError,
    #[error("user was not found")]
    InvalidUserError,
    #[error("token-cookie was not found")]
    MissingAuthError
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
            AuthError::InternalServerError(_) => HttpResponse::InternalServerError().json(self.get_response_struct()),
            AuthError::NoPermissionError => HttpResponse::Unauthorized().json(self.get_response_struct()),
            AuthError::InvalidUserError => HttpResponse::Unauthorized().json(self.get_response_struct()),
            AuthError::MissingAuthError => HttpResponse::Unauthorized().json(self.get_response_struct())
        }
    }

    fn get_response_struct(&self) -> ErrorResponse {
        ErrorResponse {
            error: self.to_string()
        }
    }
}