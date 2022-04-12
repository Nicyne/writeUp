//! Authentication and Request-processing based errors and their handling
//!
//! # Types
//!
//! + Authentication
//!     * **\[10\]** `AuthenticationError` - Occurs when accessing a secured endpoint without prior authentication
//!     * **\[11\]** `InvalidCredentialsError` - Occurs when a given set of credentials can not be processed
//!     * **\[12\]** `NoPermissionError` - Occurs when accessing a secured endpoint without sufficient authorization
//!
//! + Formal
//!     * **\[20\]** `InvalidPayloadError` - Occurs when a given payload does not match with the endpoints expectations
//!     * **\[21\]** `InvalidIDError` - Occurs when a given ID contains invalid character
//!     * **\[24\]** `InvalidInstructionsError` - Occurs when issuing an instruction that is invalid in context
//!     * **\[27\]** `InvalidInviteError` - Occurs when accessing a secured endpoint without prior authentication
//!
//! + Internal
//!     * **\[50\]** `InternalServerError` - Occurs whenever something goes wrong internally
//!     * **\[54\]** `QueryError` - Occurs whenever a query to the database fails
//!     * **\[55\]** `DBInconsistencyError` - Occurs whenever an inconsistency within the database is discovered

use actix_web::{HttpResponse, HttpResponseBuilder};
use thiserror::Error;
use serde::Serialize;
use crate::web::TIME_FORMAT;

/// Struct modelling the response-body of an error
#[derive(Serialize)]
struct ErrorResponse {
    /// Indicator whether the request was successful (will always be false)
    success: bool,
    /// Error-code indicating what went wrong
    code: i8,
    /// Specific description of what went wrong
    message: String,
    /// Timestamp of when the response was created
    time: String
}

/// Error-types that can appear in processing requests
#[derive(Error, Debug)]
pub enum APIError {
    // auth-based error
    /// An error that occurs when requesting a secured endpoint without proper authorization
    #[error("user is not logged in")]
    AuthenticationError,
    /// An error that occurs when faced with incorrect or invalid credentials
    #[error("failed to process credentials: {0}")]
    InvalidCredentialsError(String),
    /// An error that occurs when attempting something the user is not allowed to
    #[error("no permission")]
    NoPermissionError,

    // formal error
    /// An error that occurs when the payload of a request doesn't fit what the endpoint expects
    #[error("payload does not match expectations")]
    InvalidPayloadError,
    /// An error that occurs when trying to access a resource with an invalid id
    #[error("requested id contains forbidden character")]
    InvalidIDError,
    /// An error that occurs when given an instruction that is invalid in context
    #[error("invalid instruction: {0}")]
    InvalidInstructionsError(String),
    /// An error that occurs when trying to redeem an invitation with an invalid code
    #[error("invite is not valid (anymore)")]
    InvalidInviteError,

    // internal error
    /// A multipurpose error that occurs whenever something went wrong internally
    #[error("internal server error: {0}")]
    InternalServerError(String),
    /// An error that occurs whenever a query to the database failed
    #[error("query failed: {0}")]
    QueryError(String), //TODO? Add a list of failed queries
    /// An error that occurs whenever an inconsistency within the database is discovered
    #[error("database contains link ({0} -> {1}) to missing resource")]
    DBInconsistencyError(String, String)
}

impl APIError {
    /// Maps itself to the appropriate HttpResponseBuilder and an internal error-code
    fn get_response_information(&self) -> (HttpResponseBuilder, i8) {
        match self {
            // auth-based error
            APIError::AuthenticationError => (HttpResponse::Unauthorized(),10),
            APIError::InvalidCredentialsError(_) => (HttpResponse::Ok(),11),
            APIError::NoPermissionError => (HttpResponse::Forbidden(),12),
            // formal error
            APIError::InvalidPayloadError => (HttpResponse::BadRequest(),20),
            APIError::InvalidIDError => (HttpResponse::BadRequest(),21),
            APIError::InvalidInstructionsError(_) => (HttpResponse::Ok(),24),
            APIError::InvalidInviteError => (HttpResponse::Ok(),27),
            // internal error
            APIError::InternalServerError(_) => (HttpResponse::InternalServerError(),50),
            APIError::QueryError(_) => (HttpResponse::InternalServerError(), 54),
            APIError::DBInconsistencyError(_,_) => (HttpResponse::InternalServerError(),55)
        }
    }

    /// Creates a HttpResponse representing itself
    pub fn gen_response(&self) -> HttpResponse {
        let (mut response_builder, error_code) = self.get_response_information();
        response_builder.json(ErrorResponse {
            success: false,
            code: error_code,
            message: self.to_string(),
            time: chrono::Local::now().format(TIME_FORMAT).to_string()
        })
    }
}
