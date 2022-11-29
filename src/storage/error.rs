//! Database and data-handling based errors and their handling

use thiserror::Error;

/// Error-types that can appear when accessing a database
#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum DBError {
    /// An error that occurs when the supposed db-server does not respond
    #[error("could not connect to database-server")]
    ServerConnectionError,
    /// An error that occurs when issuing a query on a non existent key
    #[error("couldn't find expected ID='{1}' in table '{0}'")]
    MissingEntryError(String, String),
    /// An error that occurs when a given query fails with unknown reason
    #[error("error during query: {0}")]
    QueryError(String),
    /// An error that occurs when attempting an operation that contradicts the current state (f.e. remove a non-existent entry)
    #[error("invalid request: {0}")]
    InvalidRequestError(String),
    /// An error that occurs when encountering a character-sequence making use of characters typically used in database-operations
    #[error("sequence '{0}' contains illegal character")]
    InvalidSequenceError(String),
    /// An error that occurs when incorrect credentials have been supplied
    #[error("incorrect credentials")]
    IncorrectCredentialsError,
    /// An error that occurs when attempting a query with insufficient permission
    #[error("insufficient permission")]
    NoPermissionError,
}