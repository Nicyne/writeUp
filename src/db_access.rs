//! Contains structs and functions to access the mongodb database with

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
/// Identifier of the database inside of a mongodb-server
const DB_NAME: &str = "test";
// Collection-Identifier
/// Identifier of the collection containing all note-objects
pub const NOTES: &str = "notes";
/// Identifier of the collection containing all credential-objects
pub const CREDENTIALS: &str = "creds";
/// Identifier of the collection containing all user-objects
pub const USER: &str = "user";

// Various constants
/// Chars not serving a use outside of a potential injection-attempt
const FORBIDDEN_CHARS:[char;4] = ['{', '}', '$', ':']; //TODO? Check for '.' (only used in jwt so far)

// Schemata
// Sub-Structures
/// The individual levels of access-rights a user can have regarding a note
#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum AllowanceLevel {
    /// The user has no access to the note
    Forbidden,
    /// The user can only read the note
    Read,
    /// The user can read and modify the note
    ReadWrite,
    /// The user owns the note and therefore can read/modify/delete and share the note
    Owner
}

/// Serves as a link between a user and a note
#[derive(Debug, Serialize, Deserialize)]
pub struct Allowance {
    /// The identifier of the linked note
    pub note_id: String,
    /// The level of access the user has regarding the note
    pub level: AllowanceLevel
}

// Database-Objects
/// Structs representing the different kinds of documents to be found in the db
pub trait DatabaseObject: Serialize + DeserializeOwned + Unpin + Send + Sync {}

/// A struct modelling the required information to verify yourself as a user
#[derive(Debug, Serialize, Deserialize)]
pub struct Credential {
    /// Username
    pub _id: String,
    /// Password
    pub passwd: String //TODO DO NOT STORE PASSWORDS IN PLAIN TEXT
}
impl DatabaseObject for Credential {}
impl Credential {
    /// Compares a given password with the one associated with the account
    ///
    /// # Arguments
    ///
    /// * `passwd` - A string slice containing the supposed password in plain text
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::db_access::Credential;
    ///
    /// let cred = Credential { _id: "testUser".to_string(), passwd: "testPass".to_string() };
    ///
    /// assert!(cred.verify("testPass"));
    /// assert_eq!(cred.verify("passTest"), false);
    /// ```
    pub fn verify(&self, passwd: &str) -> bool {
        self.passwd.eq(passwd) //TODO DO NOT COMPARE PASSWORDS IN PLAIN TEXT
    }
}

/// A struct modelling a user
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// Username
    pub _id: String,
    /// A list of notes the user has access to
    pub allowances: Vec<Allowance>,
    /// A list of user this one is connected with
    pub connections: Vec<String>
}
impl DatabaseObject for User {}

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
    pub tags: Vec<String>
}
impl DatabaseObject for Note {}

// Error-Types
/// Errors that can appear when accessing the database
#[derive(Error, Debug)]
pub enum DBError {
    /// An error that occurs when the wrong credentials have been supplied
    #[error("wrong credentials")]
    WrongCredentialsError,
    /// An error that occurs when the db-server is not accessible
    #[error("could not connect to database-server")]
    ServerConnectionError,
    /// An error that occurs when a given query fails
    #[error("query returned an error")]
    QueryError,
    /// An error that occurs when the given query could not find a fitting document to return
    #[error("no document found")]
    NoDocumentFoundError
}

/// Tests a string for potential injection-attempts
///
/// # Arguments
///
/// * `str` - The string to be checked
///
/// # Examples
///
/// ```
/// use crate::db_access::is_safe;
///
/// let good_string = "Hey, i can do a whole lot here, can't i?";
/// let bad_string = "%7B%24ne%3Anull%7D"; // eq: {$ne:null}
///
/// assert!(is_safe(good_string));
/// assert!(!is_safe(bad_string));
/// ```
pub fn is_safe(str: &str) -> bool {
    for char in FORBIDDEN_CHARS {
        if str.contains(char) {
            return false
        }
    }
    return true
}

/// Attempts to create a connection to the db-server and returns it
///
/// # Arguments
///
/// * `uri` - A tuple containing both the url and the port of the db-server
/// * `cred` - A tuple containing both the username and password to login with
///
/// # Examples
///
/// ```
/// use crate::db_access::connect_to_database;
///
/// let (url, port) = ("localhost".to_string(), "27017".to_string());
/// let (username, passwd) = ("testUser".to_string(), "testPass".to_string());
///
/// let db = connect_to_database((url, port), (username, passwd)).await.unwrap();
/// ```
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

/// Searches and returns the DatabaseObject with the appropriate id
///
/// # Arguments
///
/// * `collection` - A string slice containing the collection-identifier
/// * `id` - A String containing the document-id to look for
/// * `db` - A Mutex-secured reference to the database-connection
///
/// # Examples
///
/// ```
/// use std::sync::Mutex;
/// use crate::db_access::{User, USER, connect_to_database, get_dbo_by_id};
///
/// let db = Mutex::new(connect_to_database(("localhost".to_string(), "27017".to_string()),
///     ("testUser".to_string(), "testPass".to_string())).await.unwrap());
///
/// let user: User = get_dbo_by_id::<User>(USER, "testUser".to_string(), &db).await.unwrap();
/// ```
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

/// Attempts to add a DatabaseObject to a collection, returning an Ok if successful and an Err(DBError) if not
///
/// # Arguments
///
/// * `collection` - A string slice containing the collection-identifier
/// * `obj` - The DatabaseObject to be added to the DB
/// * `db` - A Mutex-secured reference to the database-connection
///
/// # Examples
///
/// ```
/// use std::sync::Mutex;
/// use crate::db_access::{User, USER, connect_to_database, insert_dbo};
///
/// let db = Mutex::new(connect_to_database(("localhost".to_string(), "27017".to_string()),
///     ("testUser".to_string(), "testPass".to_string())).await.unwrap());
/// let user = User {
///     _id: "testUser".to_string(),
///     allowances: Vec::new(),
///     connections: Vec::new()
/// };
///
/// insert_dbo::<User>(USER, user, &db).await;
/// ```
pub async fn insert_dbo<T: DatabaseObject>(collection: &str, obj: &T, db: &Mutex<Database>) -> Result<InsertOneResult, DBError> {
    let coll = db.lock().unwrap().collection::<T>(collection);
    coll.insert_one(obj, None).await.map_err(|_| DBError::QueryError)
}

/// Attempts to update a specific document in a collection, returning an Ok if successful and an Err(DBError) if not
///
/// # Arguments
///
/// * `collection` - A string slice containing the collection-identifier
/// * `id` - A String containing the document-id to look for
/// * `query` - A document describing the update-operation
/// * `db` - A Mutex-secured reference to the database-connection
///
/// # Examples
///
/// ```
/// use std::sync::Mutex;
/// use mongodb::bson::doc;
/// use crate::db_access::{User, USER, connect_to_database, update_dbo_by_id};
///
/// let db = Mutex::new(connect_to_database(("localhost".to_string(), "27017".to_string()),
///     ("testUser".to_string(), "testPass".to_string())).await.unwrap());
///
/// let query = doc! {"$set": {"connections": ["userTest"]}};
/// let other_query = doc! {"$push":
///     {"allowances": {"note_id": "7254fa970b62u3ag62dr4d3l".to_string(), "level": "Owner"}}};
///
/// update_dbo_by_id::<User>(USER, "testUser".to_string(), query, &db).await;
/// update_dbo_by_id::<User>(USER, "testUser".to_string(), other_query, &db).await;
/// ```
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

/// Attempts to delete a specific document and returns an Ok if successful and an Err(DBError) if not
///
/// # Arguments
///
/// * `collection` - A string slice containing the collection-identifier
/// * `id` - A String containing the document-id to look for
/// * `db` - A Mutex-secured reference to the database-connection
///
/// # Examples
///
/// ```
/// use std::sync::Mutex;
/// use crate::db_access::{User, USER, connect_to_database, del_dbo_by_id};
///
/// let db = Mutex::new(connect_to_database(("localhost".to_string(), "27017".to_string()),
///     ("testUser".to_string(), "testPass".to_string())).await.unwrap());
///
/// del_dbo_by_id::<User>(USER, "testUser".to_string(), &db).await;
/// ```
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

/// Compiles a list of notes shared by a certain user.
/// Returns either a vector of note_ids or a DBError if the list could not be compiled
///
/// # Arguments
///
/// * `allowed_user_id` - The identifier of the user to be searched
/// * `allowing_user_id` - The identifier of the user sharing their notes
/// * `db` - A Mutex-secured reference to the database-connection
///
/// # Examples
///
/// ```
/// use std::sync::Mutex;
/// use crate::db_access::{connect_to_database, filter_allowances_by_user_id};
///
/// let db = Mutex::new(connect_to_database(("localhost".to_string(), "27017".to_string()),
///     ("testUser".to_string(), "testPass".to_string())).await.unwrap());
///
/// filter_allowances_by_user_id("testUser", "otherUser", &db).await;
/// ```
pub async fn filter_allowances_by_user_id(allowed_user_id: &str, allowing_user_id: &str, db: &Mutex<Database>) -> Result<Vec<String>, DBError> {
    // Get the user that is to be searched
    match get_dbo_by_id::<User>(USER, allowed_user_id.to_string(), db).await {
        Ok(allowed_user) => {
            let mut matched_allowances = Vec::new();
            for allow in allowed_user.allowances {
                // If a note is owned by the user, it can't fit the criteria
                if allow.level == AllowanceLevel::Owner {
                    continue
                }
                // Else check the actual note for its owners id
                let note_result = get_dbo_by_id::<Note>(NOTES, allow.note_id.clone(), db).await;
                if note_result.is_err() {
                    return Err(note_result.err().unwrap())
                } else if note_result.unwrap().owner_id.eq(&allowing_user_id) {
                    matched_allowances.push(allow.note_id) //TODO? Add the entire allowance to generalize
                }
            }
            Ok(matched_allowances)
        }
        Err(e) => Err(e)
    }

}
