//! Implementation for tasks on the highest level of the database-schema

use async_trait::async_trait;
use chrono::Utc;
use mongodb::bson::doc;
use mongodb::Database;
use crate::storage::{gen_hash, is_safe, verify};
use crate::storage::error::DBError;
use crate::storage::interface::{DBManager, DBMeta, UserManager};
use crate::storage::mongodb::DRIVER_ID;
use crate::storage::mongodb::schema::{Credential, CREDENTIALS, USER, User};
use crate::storage::mongodb::users::MongoDBUserManager;

/// Implements the [`DBManager`]-trait
pub(in crate::storage::mongodb) struct MongoDBDatabaseManager {
    meta: DBMeta,
    database: Database
}

impl MongoDBDatabaseManager {
    pub fn new(database: Database, version: String) -> MongoDBDatabaseManager {
        MongoDBDatabaseManager {
            meta: DBMeta {
                driver_id: DRIVER_ID.to_string(),
                version
            },
            database}
    }
}

#[async_trait]
impl DBManager for MongoDBDatabaseManager {
    fn get_meta_information(&self) -> &DBMeta {
        &self.meta
    }


    async fn auth_user(&self, username: &str, password: &str) -> Result<Box<dyn UserManager>, DBError> {
        // Check for injection-attempt in the username
        if !is_safe(username) { return Err(DBError::InvalidSequenceError(username.to_string())) }

        // Get Credentials for user
        let cred_collection = self.database.collection::<Credential>(CREDENTIALS);
        let filter = doc! { "_id": username };

        let query_result = cred_collection.find_one(filter, None).await;
        if query_result.is_err() { return Err(DBError::QueryError(format!("lookup of credential with ID='{}'", username))) }

        let found_credential = query_result.unwrap();
        return match found_credential {
            Some(credential) => {
                // Verify credentials
                if verify(password, &credential.passwd_hash) {
                    // Create UserManager
                    self.get_user(&credential._id).await
                } else { Err(DBError::IncorrectCredentialsError) }
            }
            None => {
                // Hash the password to simulate a verification-system-load and return error
                gen_hash(password);
                Err(DBError::IncorrectCredentialsError)
            }
        }
    }


    async fn add_user(&self, username: &str, password: &str) -> Result<Box<dyn UserManager>, DBError> {
        let cred_collection = self.database.collection::<Credential>(CREDENTIALS);
        let user_collection = self.database.collection::<User>(USER);

        let filter = doc! { "_id": &username };

        // Check for existing user with that username
        let check_for_existing_user_query = cred_collection.find_one(filter, None).await;
        if check_for_existing_user_query.is_ok() && check_for_existing_user_query.unwrap().is_some() {
            return Err(DBError::InvalidRequestError(format!("username '{}' already exists", username)))
        }

        // Prepare documents
        let credential = Credential {
            _id: username.to_string(),
            passwd_hash: gen_hash(password)
        };
        let user = User {
            _id: username.to_string(),
            allowances: vec![],
            connections: vec![],
            member_since: Utc::now().into()
        };

        // Insert documents
        let cred_query = cred_collection.insert_one(credential, None).await;
        let user_query = user_collection.insert_one(user, None).await;

        if cred_query.is_err() || user_query.is_err() {
            return Err(DBError::QueryError(format!("insert of credential/user with ID='{}'", username)))
        }
        if cred_query.unwrap().inserted_id != user_query.as_ref().unwrap().inserted_id {
            return Err(DBError::QueryError("ids of credential- and user-document differ".to_string()))
        }

        // Return UserManager
        return Ok(self.get_user(username).await?)
    }

    async fn get_user(&self, user_id: &str) -> Result<Box<dyn UserManager>, DBError> {
        Ok(Box::new(MongoDBUserManager::new(user_id, &self.database).await?))
    }

    async fn remove_user(&self, user_id: &str) -> Result<(), DBError> {
        let user = self.get_user(user_id).await?;

        // Remove all notes and shares
        let accessible_notes = user.get_all_notes();
        for note_id in accessible_notes.await? {
            let note_query = user.remove_note(&note_id);
            match note_query.await {
                Err(DBError::NoPermissionError) | Ok(_) => {},
                Err(e) => { return Err(e) }
            }
        }

        // Remove all associations
        let associates = user.get_associates();
        for associate in associates.await? {
            user.revoke_association(&associate).await?;
        }

        // Remove user- and credential-docs
        let cred_collection = self.database.collection::<Credential>(CREDENTIALS);
        let user_collection = self.database.collection::<User>(USER);
        let filter = doc! {"_id": user_id};

        let cred_query = cred_collection.delete_one(filter.clone(), None);
        let user_query = user_collection.delete_one(filter, None);

        // Return success
        return if cred_query.await.is_ok() && user_query.await.is_ok() { Ok(()) } else {
            Err(DBError::QueryError(format!("removal of user and credential with ID='{}'", user_id)))
        }

    }
}