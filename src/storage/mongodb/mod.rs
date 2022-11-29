//! Implementations of all of the in [`interface`](crate::storage::interface) defined traits for use with MongoDB

use std::str::FromStr;
use mongodb::bson::doc;
use mongodb::{Client, Database};
use mongodb::bson::oid::ObjectId;
use mongodb::options::ClientOptions;
use crate::storage::error::DBError;
use crate::storage::error::DBError::ServerConnectionError;
use crate::storage::interface::{DBManager, ManagerPool};
use crate::storage::is_safe;
use crate::storage::mongodb::database::MongoDBDatabaseManager;
use crate::storage::mongodb::schema::{DB_NAME, META, MetaKey, MetaEntry, Note, NOTES, User, USER};

mod schema;
mod database;
mod users;
mod notes;


/// Implements the [`ManagerPool`]-trait
pub struct MongoDBPool {
    client: Client,
    version: String
}

impl MongoDBPool {
    /// Creates a new MongoDBPool-instance.
    ///
    /// Returns [`ServerConnectionError`](DBError::ServerConnectionError) if it's not able to reach the database-server.
    /// Returns [`MigrationRequiredError`](DBError::MigrationRequiredError) if the database uses an older version of the schema.
    /// Returns [`QueryError`](DBError::QueryError) if any other error occurs.
    ///
    /// # Arguments
    ///
    /// * `uri` - Tuple of Strings containing the url and port of the database-server
    /// * `cred` - Tuple of Strings containing the username and password of a database-user
    ///
    /// # Examples
    ///
    /// ```
    /// use storage::mongodb::MongoDBPool;
    ///
    /// let url = "localhost";
    /// let port = "27017";
    /// let username = "user";
    /// let password = "pass";
    ///
    /// let pool = MongoDBPool::new((url, port), (username, password)).await.unwrap();
    ///
    /// // Every interaction with the database is done through a Manager-instance
    /// // Instances should always be retrieved this way
    /// let dbmanager = pool.get_manager();
    /// ```
    pub async fn new(uri: (String, String), cred: (String, String))
        -> Result<Box<dyn ManagerPool>, DBError> {
        // Configure the connection
        let mut client_options = ClientOptions::parse(
            format!("mongodb://{}:{}@{}:{}", cred.0, cred.1, uri.0, uri.1)).await.unwrap();
        client_options.app_name = Some("writeUp".to_string());

        // Attempt to connect
        let client = Client::with_options(client_options).unwrap();
        let db = client.database(DB_NAME);

        // Test the connection
        if db.run_command(doc! {"ping": 1}, None).await.is_ok() {
            // Get the schema-version of the database
            let meta_collection = db.collection::<MetaEntry>(META);
            let version_filter = doc! { "_id": MetaKey::schema_version.to_string() };
            let version = match meta_collection.find_one(version_filter, None).await
                .map_err(|_| DBError::QueryError("lookup of database-schema-version".to_string()))? {
                Some(version) => version.value,
                None => { //Database was freshly created
                    // Populate the meta-collection
                    meta_collection.insert_one(MetaEntry {
                        _id: MetaKey::schema_version,
                        value: schema::VERSION.to_string()
                    }, None).await
                        .map_err(|_| DBError::QueryError("insert of database-schema-version".to_string()))?;

                    schema::VERSION.to_string()
                }
            };
            // Check for a schema-version-mismatch between the database and the application
            if version.ne(&schema::VERSION.to_string()) {
                return Err(DBError::MigrationRequiredError(version, schema::VERSION.to_string()))
            }

            // Return newly created Pool
            Ok(Box::new(MongoDBPool { client, version }))
        } else { Err(ServerConnectionError) }
    }
}

impl ManagerPool for MongoDBPool {
    fn get_manager(&self) -> Box<dyn DBManager> {
        Box::new(MongoDBDatabaseManager::new(self.client.database(DB_NAME).clone(),
                                             self.version.clone()))
    }
}


/// Retrieves a [`User`]-instance from a given database.
/// Returns [`InvalidSequenceError`](DBError::InvalidSequenceError) if the id contains forbidden character.
/// Returns [`MissingEntryError`](DBError::MissingEntryError) if the id doesn't exist within the database.
///
/// # Arguments
///
/// * `user_id` - A string-slice containing the user's id
/// * `database` - A borrowed Database-struct with which to look up the user
async fn get_user(user_id: &str, database: &Database) -> Result<User, DBError> {
    // Check for injection-attempt in the user-id
    if !is_safe(&user_id) { return Err(DBError::InvalidSequenceError(user_id.to_string())) }

    // Read user from database
    let user_collection = database.collection::<User>(USER);
    let filter = doc! { "_id": user_id };

    let query_result = user_collection.find_one(filter, None).await;
    if query_result.is_err() { return Err(DBError::QueryError(format!("lookup of user with ID='{}'", user_id))) }

    return match query_result.unwrap() {
        Some(user) => Ok(user),
        None => Err(DBError::MissingEntryError(USER.to_string(), user_id.to_string()))
    }
}

/// Retrieves a [`Note`]-instance from a given database.
/// Returns [`InvalidSequenceError`](DBError::InvalidSequenceError) if the id contains forbidden character.
/// Returns [`MissingEntryError`](DBError::MissingEntryError) if the id doesn't exist within the database.
///
/// # Arguments
///
/// * `note_id` - A string-slice containing the note's id
/// * `database` - A borrowed Database-struct with which to look up the user
async fn get_note(note_id: &str, database: &Database) -> Result<Note, DBError> {
    // Check for injection-attempt in the note-id
    if !is_safe(&note_id) { return Err(DBError::InvalidSequenceError(note_id.to_string())) }

    // Read note from database
    let note_collection = database.collection::<Note>(NOTES);
    let filter = doc! { "_id": ObjectId::from_str(&note_id).unwrap() };

    let query_result = note_collection.find_one(filter, None).await;
    if query_result.is_err() { return Err(DBError::QueryError(format!("lookup of note with ID='{}'", note_id))) }

    return match query_result.unwrap() {
        Some(note) => Ok(note),
        None => Err(DBError::MissingEntryError(NOTES.to_string(), note_id.to_string()))
    }
}