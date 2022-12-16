//! Various structs and traits to serve as an abstraction-layer to the actual database-driver

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use actix_session::storage::SessionStore;
use crate::storage::error::DBError;
use crate::storage::error::DBError::NoPermissionError;

/// The individual levels of access-rights a user can have regarding a note
#[derive(Debug, Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
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
    pub driver_id: String,
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

/// A pool allowing retrieval of fresh [`DBManager`]-instances
pub trait ManagerPool: Send + Sync {
    /// Retrieves a new [`DBManager`]-instance
    fn get_manager(&self) -> Box<dyn DBManager>;
}

/// A trait providing methods for basic database-management
#[async_trait]
pub trait DBManager {
    /// Returns all available meta-information about the database
    fn get_meta_information(&self) -> &DBMeta;

    /// Attempts to verify a users credentials
    ///
    /// Returns the corresponding [`UserManager`] if successful
    /// and [`IncorrectCredentialsError`](DBError::IncorrectCredentialsError) if not
    ///
    /// # Arguments
    ///
    /// * `username` - The identifier used to log in
    /// * `password` - The password used to log in
    async fn auth_user(&self, username: &str, password: &str) -> Result<Box<dyn UserManager>, DBError>;

    /// Creates and registers a new User in the database before returning its [`UserManager`]
    ///
    /// # Arguments
    ///
    /// * `username` - The new users screen-name
    /// * `password` - The new users password
    async fn add_user(&self, username: &str, password: &str) -> Result<Box<dyn UserManager>, DBError>;
    /// Looks up a user by his ID and returns a corresponding [`UserManager`] if found
    ///
    /// # Arguments
    ///
    /// * `user_id` - The internal ID of the wanted user
    async fn get_user(&self, user_id: &str) -> Result<Box<dyn UserManager>, DBError>;
    /// Purges a user from the database
    ///
    /// # Arguments
    ///
    /// * `user_id` - The internal ID of the to be removed user
    async fn remove_user(&self, user_id: &str) -> Result<(), DBError>;
}

/// A trait providing methods for basic user-management
#[async_trait]
pub trait UserManager: Send + Sync {
    /// Returns all available meta-information about the user
    fn get_meta_information(&self) -> &UserMeta;

    /// Introduce a user as an associate of this one
    async fn associate_with(&self, user_id: &str) -> Result<(), DBError>;
    /// Collects and returns a list of user_ids of user associated with this one
    async fn get_associates(&self) -> Result<Vec<String>, DBError>;
    /// Removes any association to a given user
    async fn revoke_association(&self, user_id: &str) -> Result<(), DBError>;

    /// Collects and returns a list of note_id of all the notes this user has access to
    async fn get_all_notes(&self) -> Result<Vec<String>, DBError>;
    /// Creates and saves a new note in the database before returning its [`NoteManager`]
    ///
    /// # Arguments
    ///
    /// * `title` - The new notes title
    async fn add_note(&self, title: &str) -> Result<Box<dyn NoteManager>, DBError>;
    /// Looks up a note by its ID and returns the corresponding [`NoteManager`] if found
    ///
    /// # Arguments
    ///
    /// * `note_id` - The ID of the wanted note
    async fn get_note(&self, note_id: &str) -> Result<Box<dyn NoteManager>, DBError>;
    /// Removes a note from the database
    ///
    /// # Arguments
    ///
    /// * `note_id` - The ID of the to be removed note
    async fn remove_note(&self, note_id: &str) -> Result<(), DBError>;
}

/// A trait providing methods for basic note-management
#[async_trait]
pub trait NoteManager: Send + Sync {
    /// Returns all available meta-information about the database
    fn get_meta_information(&self) -> &NoteMeta;

    /// Returns the note's title
    async fn get_title(&self) -> Result<String, DBError>;
    /// Sets the note's title to the given string
    ///
    /// # Arguments
    ///
    /// * `title` - The new title of the note
    async fn set_title(&mut self, title: String) -> Result<(), DBError>;

    /// Returns the note's contents
    async fn get_content(&self) -> Result<String, DBError>;
    /// Sets the note's content to the given string
    ///
    /// # Arguments
    ///
    /// * `content` - The new content of the note
    async fn set_content(&mut self, content: String) -> Result<(), DBError>;

    /// Returns a list of all tags associated with the note
    async fn get_tags(&self) -> Result<Vec<String>, DBError>;
    /// Sets the note to exclusively associate itself with a given list of tags
    ///
    /// # Arguments
    ///
    /// * `tags` - List of all the tags that are to be associated with this note
    async fn set_tags(&mut self, tags: Vec<String>) -> Result<(), DBError>;

    /// Sets the permissions of a given user regarding this note
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user who's permissions are to be modified
    /// * `perm` - The new [`PermissionLevel`]
    async fn update_share(&self, user_id: &str, perm: PermissionLevel) -> Result<(), DBError>;

    /// Compares the prevailing [`PermissionLevel`] to the one required.
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

// Session-store
/// Trait that extends [`SessionStore`] with `Send` and `Sync` to allow for use in multithreaded scenarios
pub trait SendableSessionStore: SessionStore + Send + Sync {}
