//! Database and data-handling based errors and their handling

use thiserror::Error;

/// Error-types that can appear when accessing a database
#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum DBError {
    // general
    /// An error that occurs when the supposed db-server does not respond
    #[error("could not connect to database-server")]
    ServerConnectionError,
    /// An error that occurs when issuing a query on a non existent key
    #[error("couldn't find expected {0} with ID={1}")]
    MissingEntryError(String, String),
    /// An error that occurs when a given query fails with unknown reason
    #[error("query returned an error")]
    QueryError,

    // return-based
    /// An error that occurs when the given query could not find a fitting entry to return
    #[error("no result found")]
    NoResultError,
    /// An error that occurs when the returned data does not follow the expected schema
    #[error("failed to parse to schema")]
    SchemaParseError,

    // input-based
    /// An error that occurs when trying to input a string containing [forbidden character](crate::storage::FORBIDDEN_CHARS)
    #[error("sequence uses forbidden characters")]
    InvalidSequenceError,
    /// An error that occurs when trying to add an entry with an already existing key in the database
    #[error("entry already exists")]
    DuplicateError,

    // various
    /// An error that occurs when incorrect credentials have been supplied
    #[error("incorrect credentials")]
    IncorrectCredentialsError,
    /// An error that occurs when attempting a query with insufficient permission
    #[error("insufficient permission")]
    NoPermissionError,
}