use async_trait::async_trait;
use chrono::Utc;
use mongodb::bson::doc;
use mongodb::Database;
use crate::storage::{Driver, gen_hash, verify};
use crate::storage::error::DBError;
use crate::storage::interface::{DBManager, DBMeta, UserManager};
use crate::storage::mongodb::schema;
use crate::storage::mongodb::schema::{Credential, CREDENTIALS, USER, User};
use crate::storage::mongodb::users::MongoDBUserManager;

pub(in crate::storage::mongodb) struct MongoDBDatabaseManager {
    meta: DBMeta,
    database: Database
}

impl MongoDBDatabaseManager {
    pub fn new(database: Database) -> MongoDBDatabaseManager {
        MongoDBDatabaseManager {
            meta: DBMeta {
                driver: Driver::MongoDB,
                version: schema::VERSION.to_string()
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
        // Get Credentials for user
        let cred_collection = self.database.collection::<Credential>(CREDENTIALS);
        let filter = doc! { "_id": username };

        let query_result = cred_collection.find_one(filter, None).await;
        if query_result.is_err() { return Err(DBError::QueryError) }

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
            return Err(DBError::QueryError)
        }
        if cred_query.unwrap().inserted_id != user_query.as_ref().unwrap().inserted_id {
            return Err(DBError::QueryError)
        }

        // Return UserManager
        return Ok(self.get_user(&user_query.unwrap().inserted_id.to_string()).await?)
    }

    async fn get_user(&self, user_id: &str) -> Result<Box<dyn UserManager>, DBError> {
        Ok(Box::new(MongoDBUserManager::new(user_id, &self.database).await?))
    }

    async fn remove_user(&self, user_id: &str) -> Result<(), DBError> {
        let user = self.get_user(user_id).await?;

        let accessible_notes = user.get_all_notes();
        for note_id in accessible_notes.await? {
            let note_query = user.remove_note(&note_id);
            match note_query.await {
                Err(DBError::NoPermissionError) | Ok(_) => {},
                Err(e) => { return Err(e) }
            }
        }

        let associates = user.get_associates();
        for associate in associates.await? {
            user.revoke_association(&associate).await?;
        }

        let cred_collection = self.database.collection::<Credential>(CREDENTIALS);
        let user_collection = self.database.collection::<User>(USER);
        let filter = doc! {"_id": user_id};

        let cred_query = cred_collection.delete_one(filter.clone(), None);
        let user_query = user_collection.delete_one(filter, None);

        return if cred_query.await.is_ok() && user_query.await.is_ok() { Ok(()) } else { Err(DBError::QueryError) }

    }
}