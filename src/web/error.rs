//! Authentication and Request-processing based errors and their handling

use actix_web::HttpResponse;
use thiserror::Error;
use serde::Serialize;

/// Error-types that can appear in processing requests
#[derive(Error, Debug)]
pub enum AuthError {
    /// An error that occurs when the wrong credentials have been supplied
    #[error("wrong credentials")]
    WrongCredentialsError,
    /// An error that occurs when an invalid (expired) JWT gets supplied
    #[error("jwt token not valid")]
    JWTTokenError,
    /// A multipurpose error that occurs whenever something went wrong internally
    #[error("internal server error: {0}")]
    InternalServerError(String),
    /// An error that occurs when access to a resource gets requested without proper clearance
    #[error("no permission")]
    NoPermissionError,
    /// An error that occurs when supplied with a valid JWT of a removed user-account
    #[error("user was not found")]
    InvalidUserError,
    /// An error that occurs when no JWT gets supplied
    #[error("token-cookie was not found")]
    MissingAuthError
}

/// Struct modelling the response-body of an error
#[derive(Serialize)]
struct ErrorResponse {
    error: String
}

impl AuthError {
    /// Creates a HttpResponse representing itself
    pub fn gen_response(&self) -> HttpResponse {
        match self {
            AuthError::WrongCredentialsError => HttpResponse::Unauthorized().json(self.get_response_struct()),
            AuthError::JWTTokenError => HttpResponse::Unauthorized().json(self.get_response_struct()),
            AuthError::InternalServerError(_) => HttpResponse::InternalServerError().json(self.get_response_struct()),
            AuthError::NoPermissionError => HttpResponse::Forbidden().json(self.get_response_struct()),
            AuthError::InvalidUserError => HttpResponse::Unauthorized().json(self.get_response_struct()),
            AuthError::MissingAuthError => HttpResponse::Unauthorized().json(self.get_response_struct())
        }
    }

    /// Creates a ErrorResponse-object representing itself
    fn get_response_struct(&self) -> ErrorResponse {
        ErrorResponse {
            error: self.to_string()
        }
    }
}