use std::str::FromStr;
use mongodb::bson::doc;
use mongodb::{Client, Database};
use mongodb::bson::oid::ObjectId;
use mongodb::options::ClientOptions;
use crate::storage::error::DBError;
use crate::storage::error::DBError::ServerConnectionError;
use crate::storage::interface::{DBManager, ManagerPool};
use crate::storage::mongodb::database::MongoDBDatabaseManager;
use crate::storage::mongodb::schema::{DB_NAME, Note, NOTES, User, USER};

mod schema;
mod database;
mod users;
mod notes;

pub struct MongoDBPool {
    client: Client
}

impl MongoDBPool {
    pub async fn new(uri: (String, String), cred: (String, String))
        -> Result<Box<dyn ManagerPool>, DBError> {
        // Configure the connection
        let mut client_options = ClientOptions::parse(
            format!("mongodb://{}:{}@{}:{}", cred.0, cred.1, uri.0, uri.1)).await.unwrap();
        client_options.app_name = Some("writeUp".to_string());

        // Attempt to connect
        let client = Client::with_options(client_options).unwrap();
        let db = client.database(schema::DB_NAME);

        // Test the connection
        if db.run_command(doc! {"ping": 1}, None).await.is_ok() {
            Ok(Box::new(MongoDBPool { client }))
        } else { Err(ServerConnectionError) }
    }
}

impl ManagerPool for MongoDBPool {
    fn get_manager(&self) -> Box<dyn DBManager> {
        Box::new(MongoDBDatabaseManager::new(self.client.database(DB_NAME).clone()))
    }
}

async fn get_user(user_id: &str, database: &Database) -> Result<User, DBError> {
    let user_collection = database.collection::<User>(USER);
    let filter = doc! { "_id": user_id };

    let query_result = user_collection.find_one(filter, None).await;
    if query_result.is_err() { return Err(DBError::QueryError) }

    return match query_result.unwrap() {
        Some(user) => Ok(user),
        None => Err(DBError::MissingEntryError("User".to_string(), user_id.to_string()))
    }
}

async fn get_note(note_id: &str, database: &Database) -> Result<Note, DBError> {
    let note_collection = database.collection::<Note>(NOTES);
    let filter = doc! { "_id": ObjectId::from_str(&note_id).unwrap() };

    let query_result = note_collection.find_one(filter, None).await;
    if query_result.is_err() { return Err(DBError::QueryError) }

    return match query_result.unwrap() {
        Some(note) => Ok(note),
        None => Err(DBError::MissingEntryError("Note".to_string(), note_id.to_string()))
    }
}