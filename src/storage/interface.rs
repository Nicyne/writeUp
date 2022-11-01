//! Various structs and traits to serve as an abstraction-layer to the actual database-driver

use chrono::{DateTime, Utc};
use crate::storage::Driver;
use crate::storage::error::DBError;
use crate::storage::error::DBError::NoPermissionError;

/// The individual levels of access-rights a user can have regarding a note
#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub enum PermissionLevel {
    /// The user has no access to the note
    Forbidden,
    /// The user can only read the note
    Read,
    /// The user can read and modify the note
    ReadWrite,
    /// The user can read, modify, delete and share the note
    Moderate
}

// Meta-Data

/// A struct modelling a database's meta-information
pub struct DBMeta {
    /// The database-driver that is being utilised
    pub driver: Driver,
    /// The current schema-version
    pub version: String
}

/// A struct modelling a user's meta-information
pub struct UserMeta {
    /// ID
    pub id: String,
    /// Screen-name
    pub name: String,
    /// Timestamp from when the user was created
    pub member_since: DateTime<Utc>
}

/// A struct modelling a note's meta-information
pub struct NoteMeta {
    /// The ID
    pub id: String,
    /// The title
    pub title: String,
    /// The level of access to this note
    pub permission: PermissionLevel,
    /// The ID of the user owning this note
    pub owner_id: String,
    /// The timestamp from when the note was created
    pub created_at: DateTime<Utc>,
    /// The timestamp from when the note was last modified
    pub last_edited: DateTime<Utc>
}

// Manager

/// A trait providing methods for basic database-management
pub trait DBManager {
    /// Returns all available meta-information about the database
    fn get_meta_information(&self) -> DBMeta;

    /// Attempts to verify a users credentials
    ///
    /// Returns the corresponding [`UserManager`] if successful
    /// and `IncorrectCredentialsError` if not
    ///
    /// # Arguments
    ///
    /// * `username` - The identifier used to log in
    /// * `password` - The password used to log in
    fn auth_user<User: UserManager>(&self, username: String, password: String) -> Result<User, DBError>;

    /// Creates and registers a new User in the database before returning its [`UserManager`]
    ///
    /// # Arguments
    ///
    /// * `username` - The new users screen-name
    /// * `password` - The new users password
    fn add_user<User: UserManager>(&self, username: String, password: String) -> Result<User, DBError>;
    /// Looks up a user by his ID and returns a corresponding [`UserManager`] if found
    ///
    /// # Arguments
    ///
    /// * `user_id` - The internal ID of the wanted user
    fn get_user<User: UserManager>(&self, user_id: String) -> Result<User, DBError>;
    /// Purges a user from the database
    ///
    /// # Arguments
    ///
    /// * `user_id` - The internal ID of the to be removed user
    fn remove_user(&self, user_id: String) -> Result<(), DBError>;
}

/// A trait providing methods for basic user-management
pub trait UserManager {
    /// Returns all available meta-information about the user
    fn get_meta_information(&self) -> UserMeta;

    /// Collects and returns a list of user_ids of user associated with this one
    fn get_associates(&self) -> Result<Vec<String>, DBError>;

    /// Collects and returns a list of all notes the user has access to
    fn get_all_notes<Note: NoteManager>(&self) -> Result<Vec<Note>, DBError>;
    /// Creates and saves a new note in the database before returning its [`NoteManager`]
    ///
    /// # Arguments
    ///
    /// * `title` - The new notes title
    fn add_note<Note: NoteManager>(&self, title: String) -> Result<Note, DBError>;
    /// Looks up a note by its ID and returns the corresponding [`NoteManager`] if found
    ///
    /// # Arguments
    ///
    /// * `note_id` - The ID of the wanted note
    fn get_note<Note: NoteManager>(&self, note_id: String) -> Result<Note, DBError>;
    /// Removes a note from the database
    ///
    /// # Arguments
    ///
    /// * `note_id` - The ID of the to be removed note
    fn remove_note(&self, note_id: String) -> Result<(), DBError>;
}

/// A trait providing methods for basic note-management
pub trait NoteManager {
    /// Returns all available meta-information about the database
    fn get_meta_information(&self) -> NoteMeta;

    /// Returns the note's contents
    fn get_content(&self) -> Result<String, DBError>;
    /// Sets the note's content to the given string
    ///
    /// # Arguments
    ///
    /// * `content` - The new content of the note
    fn set_content(&self, content: String) -> Result<(), DBError>;

    /// Returns a list of all tags associated with the note
    fn get_tags(&self) -> Result<Vec<String>, DBError>;
    /// Sets the note to exclusively associate itself with a given list of tags
    ///
    /// # Arguments
    ///
    /// * `tags` - List of all the tags that are to be associated with this note
    fn set_tags(&self, tags: Vec<String>) -> Result<(), DBError>;

    /// Compares the prevailing [PermissionLevel] to the one required.
    ///
    /// Returns [`NoPermissionError`] if permissions are insufficient
    ///
    /// # Arguments
    ///
    /// * `req_perm` - The minimal [`PermissionLevel`] that is required
    fn check_permission(&self, req_perm: PermissionLevel) -> Result<(), DBError> {
        return if self.get_meta_information().permission >= req_perm { Ok(()) } else { Err(NoPermissionError) }
    }
}