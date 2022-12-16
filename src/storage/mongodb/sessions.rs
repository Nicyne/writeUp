//! Implementation of the [`SessionStore`]-trait of the actix-framework
//! to allow for sessions to be stored independently other database-solutions

use std::collections::HashMap;
use actix_session::storage::{LoadError, SaveError, SessionKey, SessionStore, UpdateError};
use actix_web::cookie::time::Duration;
use async_trait::async_trait;
use chrono::Utc;
use log::warn;
use mongodb::Database;
use mongodb::bson::{DateTime, doc};
use rand::distributions::Alphanumeric;
use rand::Rng;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use crate::storage::error::DBError;
use crate::storage::interface::SendableSessionStore;
use crate::storage::mongodb::get_client;
use crate::storage::mongodb::schema::DB_NAME;

/// Identifier of the collection containing all note-objects
const SESSIONS: &str = "sessions";

/// A struct modelling a session
#[derive(Debug, Serialize, Deserialize)]
struct Session {
    /// The key identifying the session
    _id: String,
    /// The state of the session
    state: HashMap<String, String>,
    /// The expiration-date for the session
    expire: DateTime
}

/// Implements the [`SendableSessionStore`]-trait
#[derive(Clone)]
pub struct MongoDBSessionStore {
    database: Database
}

impl MongoDBSessionStore {
    /// Creates a new instance of [`MongoDBSessionStore`]
    ///
    /// # Arguments
    ///
    /// * `uri` - Tuple of Strings containing the url and port of the database-server
    /// * `cred` - Tuple of Strings containing the username and password of a database-user
    pub async fn new(uri: (String, String), cred: (String, String)) -> MongoDBSessionStore {
        // Connect to database
        let client = get_client(uri, cred).await.unwrap();
        let database = client.database(DB_NAME);

        // Clean up any timed out sessions
        let collection = database.collection::<Session>(SESSIONS);
        let timeout_filter = doc! { "expire": { "$lt": DateTime::from(Utc::now()) } };
        if collection.delete_many(timeout_filter, None).await.is_err() {
            warn!("Failed to remove timed-out sessions from storage")
        }

        // Return SessionStore
        MongoDBSessionStore {
            database
        }
    }
}

#[async_trait(?Send)]
impl SessionStore for MongoDBSessionStore {
    async fn load(&self, session_key: &SessionKey) -> Result<Option<HashMap<String, String>>, LoadError> {
        let session_id = session_key.as_ref();
        let filter = doc! { "_id": session_id };

        let collection = self.database.collection::<Session>(SESSIONS);
        match collection.find_one(filter.clone(), None).await {
            Ok(Some(session)) =>
                // Verify that the session is still valid
                Ok(if session.expire > DateTime::from(Utc::now()) {
                    Some(session.state)
                } else {
                    // Remove session from storage
                    if collection.delete_one(filter, None).await.is_err() {
                        warn!("Failed to remove timed-out session from storage")
                    };
                    None
                }),
            Ok(None) => Ok(None),
            Err(_) => Err(LoadError::Other(DBError::QueryError("lookup of a session".to_string()).into()))
        }
    }

    async fn save(&self, session_state: HashMap<String, String>, ttl: &Duration) -> Result<SessionKey, SaveError> {
        let session_key = generate_session_key();
        let session = Session {
            _id: session_key.as_ref().to_string(),
            state: session_state,
            expire: get_expiration_date(ttl)
        };

        let collection = self.database.collection::<Session>(SESSIONS);
        match collection.insert_one(session, None).await {
            Ok(_) => Ok(session_key),
            Err(_) => Err(SaveError::Other(DBError::QueryError("insertion of new session".to_string()).into()))
        }
    }

    async fn update(&self, session_key: SessionKey, session_state: HashMap<String, String>, ttl: &Duration) -> Result<SessionKey, UpdateError> {
        let session_id = session_key.as_ref();
        let filter = doc! { "_id": session_id };
        let state_json = serde_json::to_string(&session_state).unwrap();

        let query = doc! { "$set": { "state": state_json, "expire": get_expiration_date(ttl) } };

        let collection = self.database.collection::<Session>(SESSIONS);
        match collection.update_one(filter, query, None).await {
            Ok(_) => Ok(session_key),
            Err(_) => Err(UpdateError::Other(DBError::QueryError("update of session-specifics".to_string()).into()))
        }
    }

    async fn update_ttl(&self, session_key: &SessionKey, ttl: &Duration) -> Result<(), anyhow::Error> {
        let session_id = session_key.as_ref();
        let filter = doc! { "_id": session_id };

        let query = doc! { "$set": { "expire": get_expiration_date(ttl) } };

        let collection = self.database.collection::<Session>(SESSIONS);
        match collection.update_one(filter, query, None).await {
            Ok(_) => Ok(()),
            Err(_) => Err(DBError::QueryError("update of session-ttl".to_string()).into())
        }
    }

    async fn delete(&self, session_key: &SessionKey) -> Result<(), anyhow::Error> {
        let session_id = session_key.as_ref();
        let filter = doc! { "_id": session_id };

        let collection = self.database.collection::<Session>(SESSIONS);
        match collection.delete_one(filter, None).await {
            Ok(_) => Ok(()),
            Err(_) => Err(DBError::QueryError("removal of session-entry".to_string()).into())
        }
    }
}

impl SendableSessionStore for MongoDBSessionStore {}

/// Calculates the time 'ttl'-amount of time from now
fn get_expiration_date(ttl: &Duration) -> DateTime {
    DateTime::from(Utc::now() + chrono::Duration::seconds(ttl.whole_seconds()))
}

// https://github.com/actix/actix-extras/blob/1774b8a36ef145bb85e2c0db50c9ff78f844804e/actix-session/src/storage/utils.rs
/// Session key generation routine that follows [OWASP recommendations].
///
/// [OWASP recommendations]: https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html#session-id-entropy
fn generate_session_key() -> SessionKey {
    let value = std::iter::repeat(())
        .map(|()| OsRng.sample(Alphanumeric))
        .take(64)
        .collect::<Vec<_>>();

    String::from_utf8(value).unwrap().try_into().unwrap()
}