//! The schema used within the database

use std::fmt;
use std::fmt::Formatter;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
use crate::storage::interface::PermissionLevel;

/// Schema-version
pub const VERSION: &str = "1.0";

// Database-Identifier
/// Identifier of the database inside of a mongodb-server
pub(in crate::storage) const DB_NAME: &str = "writeup";
// Collection-Identifier
/// Identifier of the collection containing all meta-information
pub const META: &str = "meta";
/// Identifier of the collection containing all note-objects
pub const NOTES: &str = "notes";
/// Identifier of the collection containing all credential-objects
pub const CREDENTIALS: &str = "creds";
/// Identifier of the collection containing all user-objects
pub const USER: &str = "user";

/// The various meta-information in form of its id
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub enum MetaKey {
    /// The version-number of the schema in-use
    schema_version
}
impl fmt::Display for MetaKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// A struct modelling a simple key-value relationship as used in the meta-collection
#[derive(Debug, Serialize, Deserialize)]
pub struct MetaEntry {
    pub _id: MetaKey,
    pub value: String
}

/// Serves as a link between a user and a note
#[derive(Debug, Serialize, Deserialize)]
pub struct Allowance {
    /// The identifier of the linked note
    pub note_id: String,
    /// The level of access the user has regarding the note
    pub level: PermissionLevel,
    /// The ID of the owner of the allowed note
    pub owner_id: String
}

/// A struct modelling the required information to verify yourself as a user
#[derive(Debug, Serialize, Deserialize)]
pub struct Credential {
    /// Username
    pub _id: String,
    /// Password-Hash
    pub(in crate::storage::mongodb) passwd_hash: String
}

/// A struct modelling a user
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// Username
    pub _id: String,
    /// A list of notes the user has access to
    pub allowances: Vec<Allowance>,
    /// A list of user this one is connected with
    pub connections: Vec<String>,
    /// The timestamp of this user's creation
    pub member_since: DateTime
}

/// A struct modelling a note
#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    /// The title
    pub title: String,
    /// The actual note
    pub content: String,
    /// The user owning this note
    pub owner_id: String,
    /// The tags associated with this note
    pub tags: Vec<String>,
    /// The timestamp of this note's creation
    pub created_at: DateTime,
    /// The timestamp of this note's last modification
    pub last_edited: DateTime
}