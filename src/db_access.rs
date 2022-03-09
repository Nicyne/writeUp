use mongodb::{Client, Database};
use mongodb::bson::{Bson, doc, Document};
use mongodb::options::ClientOptions;
use thiserror::Error;
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use crate::db_access::DBError::{NoDocumentFoundError, QueryError, ServerConnectionError};
use std::str::FromStr;
use std::sync::Mutex;
use mongodb::bson::oid::ObjectId;
use mongodb::results::{DeleteResult, InsertOneResult};

// Database-Identifier
const DB_NAME: &str = "test";
// Collection-Identifier
pub const NOTES: &str = "notes";
pub const CREDENTIALS: &str = "creds";
pub const USER: &str = "user";

// Schemata
// Sub-Structures
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum AllowanceLevel {
    Read,
    ReadWrite,
    Owner
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Allowance {
    pub note_id: String,
    pub level: AllowanceLevel
}

// Database-Objects
pub trait DatabaseObject: Serialize + DeserializeOwned + Unpin + Send + Sync {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Credential {
    pub _id: String,
    pub passwd: String //TODO DO NOT STORE PASSWORDS IN PLAIN TEXT
}
impl DatabaseObject for Credential {}
impl Credential {
    pub fn verify(&self, passwd: &str) -> bool {
        self.passwd.eq(passwd) //TODO DO NOT COMPARE PASSWORDS IN PLAIN TEXT
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: String,
    pub allowances: Vec<Allowance>,
    pub connections: Vec<String>
}
impl DatabaseObject for User {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub title: String,
    pub content: String,
    pub owner_id: String,
    pub tags: Vec<String>
}
impl DatabaseObject for Note {}

// Error-Types
#[derive(Error, Debug)]
pub enum DBError {
    #[error("wrong credentials")]
    WrongCredentialsError,
    #[error("could not connect to database-server")]
    ServerConnectionError,
    #[error("query returned an error")]
    QueryError,
    #[error("no document found")]
    NoDocumentFoundError
}

pub async fn connect_to_database(uri: (String, String), cred: (String, String)) -> Result<Database, DBError> {
    // Configure the connection
    let mut client_options = ClientOptions::parse(format!("mongodb://{}:{}@{}:{}", cred.0, cred.1, uri.0, uri.1)).await.unwrap();
    client_options.app_name = Some("writeUp".to_string());
    // Attempt to connect
    let client = Client::with_options(client_options).unwrap();
    let db = client.database(DB_NAME);
    // Test the connection
    db.run_command(doc! {"ping": 1}, None).await.map(|_|db).map_err(|_|ServerConnectionError)
}

pub async fn get_dbo_by_id<T: DatabaseObject>(collection: &str, id: String, db: &Mutex<Database>) -> Result<T,DBError> {
    // Prepare the query
    let coll = db.lock().unwrap().collection::<T>(collection);
    let filter;
    if collection.eq(NOTES) {
        filter = doc! {"_id": ObjectId::from_str(id.as_str()).unwrap()};
    } else {
        filter = doc! {"_id": id};
    }
    // Match the result
    match coll.find_one(filter, None).await {
        Ok(Some(doc)) => Ok(doc),
        Ok(None) => Err(NoDocumentFoundError),
        Err(_) => Err(QueryError)
    }
}

pub async fn insert_dbo<T: DatabaseObject>(collection: &str, obj: &T, db: &Mutex<Database>) -> Result<InsertOneResult, DBError> {
    let coll = db.lock().unwrap().collection::<T>(collection);
    coll.insert_one(obj, None).await.map_err(|_| DBError::QueryError)
}

pub async fn update_dbo_by_id<T: DatabaseObject>(collection: &str, id: String, query: Document, db: &Mutex<Database>) -> Result<Option<Bson>, DBError> {
    let coll = db.lock().unwrap().collection::<T>(collection);
    let filter;
    if collection.eq(NOTES) {
        filter = doc! {"_id": ObjectId::from_str(id.as_str()).unwrap()};
    } else {
        filter = doc! {"_id": id};
    }
    match coll.update_one(filter, query, None).await {
        Ok(res) => Ok(res.upserted_id),
        Err(_) => Err(DBError::QueryError)
    }
}

pub async fn del_dbo_by_id<T: DatabaseObject>(collection: &str, id: String, db: &Mutex<Database>) -> Result<DeleteResult,DBError> {
    // Prepare the query
    let coll = db.lock().unwrap().collection::<T>(collection);
    let filter;
    if collection.eq(NOTES) {
        filter = doc! {"_id": ObjectId::from_str(id.as_str()).unwrap()};
    } else {
        filter = doc! {"_id": id};
    }
    // Match the result
    match coll.delete_one(filter, None).await {
        Ok(res) => Ok(res),
        Err(_) => Err(QueryError)
    }
}
