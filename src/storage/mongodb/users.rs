//! Implementation for tasks regarding a specific user

use std::str::FromStr;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use crate::storage::error::DBError;
use crate::storage::interface::{NoteManager, PermissionLevel, UserManager, UserMeta};
use crate::storage::is_safe;
use crate::storage::mongodb::get_user;
use crate::storage::mongodb::notes::MongoDBNoteManager;
use crate::storage::mongodb::schema::{Note, NOTES, User, USER};

/// Implements the [`UserManager`]-trait
pub(in crate::storage::mongodb) struct MongoDBUserManager {
    meta: UserMeta,
    database: Database
}

impl MongoDBUserManager {
    pub async fn new(username: &str, db: &Database) -> Result<MongoDBUserManager, DBError> {
        let user = get_user(username, db).await?;
        Ok(MongoDBUserManager {
            meta: UserMeta {
                id: username.to_string(),
                name: user._id,
                member_since: DateTime::from(user.member_since)
            },
            database: db.clone()
        })
    }
}

#[async_trait]
impl UserManager for MongoDBUserManager {
    fn get_meta_information(&self) -> &UserMeta {
        &self.meta
    }


    async fn associate_with(&self, user_id: &str) -> Result<(), DBError> {
        // Sanity-checks
        if !is_safe(user_id) { return Err(DBError::InvalidSequenceError(user_id.to_string())) }
        if self.meta.id.eq(&user_id) {
            return Err(DBError::InvalidRequestError("users can not be associated with themselves".to_string()))
        }
        let user = get_user(&user_id, &self.database).await?;
        if user.connections.contains(&self.meta.id) {
            return Err(DBError::InvalidRequestError(format!("'{}' and '{}' already share an association with each other", self.meta.id, user_id)))
        }

        // Add user to each others connection-field
        let user_collection = self.database.collection::<User>(USER);

        let query_one = user_collection.update_one(doc! {"_id": &user._id},
                                                   doc! {"$push": {"connections": &self.meta.id}},
                                                   None).await;
        let query_two = user_collection.update_one(doc! {"_id": &self.meta.id},
                                                   doc! {"$push": {"connections": &user._id}},
                                                   None).await;

        return if query_one.is_ok() && query_two.is_ok() {Ok(())} else {
            Err(DBError::QueryError(format!("adding of each others user_id to own entry ('{}' <-> '{}')", &self.meta.id, &user._id)))
        }
    }

    async fn get_associates(&self) -> Result<Vec<String>, DBError> {
        return Ok(get_user(&self.meta.id, &self.database).await?.connections)
    }

    async fn revoke_association(&self, user_id: &str) -> Result<(), DBError> {
        if !is_safe(user_id) { return Err(DBError::InvalidSequenceError(user_id.to_string())) }
        // Check for existing association
        if !self.get_associates().await?.contains(&user_id.to_string()) {
            return Err(DBError::InvalidRequestError(format!("'{}' and '{}' are not associated with each other", self.meta.id, user_id)))
        }

        // Collect all allowances between the user
        let user_one = get_user(&self.meta.id, &self.database).await?;
        let user_two = get_user(&user_id, &self.database).await?;

        let user_one_shares: Vec<String> = user_one.allowances.iter()
            .filter(|allow| allow.owner_id.eq(&user_two._id))
            .map(|allow| allow.note_id.clone()).collect();
        let user_two_shares: Vec<String> = user_two.allowances.iter()
            .filter(|allow| allow.owner_id.eq(&user_one._id))
            .map(|allow| allow.note_id.clone()).collect();

        // Remove all allowances between the user
        let user_collection = self.database.collection::<User>(USER);

        let query_rm_allow_user_one = user_collection.update_one(doc! {"_id": &user_one._id},
                                                                 doc! {"$pull": {"allowances": {"note_id": {"$in": user_one_shares}}}},
                                                                 None).await;
        let query_rm_allow_user_two = user_collection.update_one(doc! {"_id": &user_two._id},
                                                                 doc! {"$pull": {"allowances": {"note_id": {"$in": user_two_shares}}}},
                                                                 None).await;
        if query_rm_allow_user_one.is_err() || query_rm_allow_user_two.is_err() { return Err(DBError::QueryError("revoke allowances for shared notes".to_string())) }

        // Remove association
        let query_rm_assoc_user_one = user_collection.update_one(doc! {"_id": &user_one._id},
                                                                 doc! {"$pull": {"connections": &user_two._id}},
                                                                 None).await;
        let query_rm_assoc_user_two = user_collection.update_one(doc! {"_id": &user_two._id},
                                                                 doc! {"$pull": {"connections": &user_one._id}},
                                                                 None).await;

        return if query_rm_assoc_user_one.is_ok() && query_rm_assoc_user_two.is_ok() { Ok(()) } else {
            Err(DBError::QueryError(format!("cancel association-status between '{}' and '{}'", user_one._id, user_two._id)))
        }
    }


    async fn get_all_notes(&self) -> Result<Vec<String>, DBError> {
        let user = get_user(&self.meta.id, &self.database).await?;
        return Ok(user.allowances.iter().map(|allow| allow.note_id.clone()).collect())
    }

    async fn add_note(&self, title: &str) -> Result<Box<dyn NoteManager>, DBError> {
        let new_note = Note {
            title: title.to_string(),
            content: "".to_string(),
            owner_id: self.meta.id.clone(),
            tags: vec![],
            created_at: Utc::now().into(),
            last_edited: Utc::now().into()
        };

        let note_collection = self.database.collection::<Note>(NOTES);
        let user_collection = self.database.collection::<User>(USER);

        let note_query = note_collection.insert_one(new_note, None).await;
        let filter = doc! { "_id": &self.meta.id };
        let note_id = note_query.map_err(|_| DBError::QueryError("insert of new note".to_string()))?.inserted_id.as_object_id().unwrap().to_string();

        let user_query = user_collection.update_one(filter,
                                                    doc! {"$push": {"allowances": {"note_id": &note_id, "level": "Moderate", "owner_id": &self.meta.id}}}, //TODO Serialize permission from enum
                                                    None).await;
        return if user_query.is_ok() { Ok(self.get_note(&note_id).await?) } else {
            Err(DBError::QueryError(format!("register new note with owner '{}'", self.get_meta_information().id)))
        }
    }

    async fn get_note(&self, note_id: &str) -> Result<Box<dyn NoteManager>, DBError> {
        let user = get_user(&self.meta.id, &self.database).await?;
        return match user.allowances.iter().find(|allow| allow.note_id == note_id) {
            Some(allowance) => {
                Ok(Box::new(MongoDBNoteManager::new(note_id,
                                                    allowance.level,
                                                    &self.database).await?))
            },
            None => Err(DBError::NoPermissionError)
        }
    }

    async fn remove_note(&self, note_id: &str) -> Result<(), DBError> {
        // Verify the note exists and the user is allowed to delete it
        let note = self.get_note(&note_id).await?;
        if note.get_meta_information().permission < PermissionLevel::Moderate { return Err(DBError::NoPermissionError) }

        // Remove all references by user allowance
        let user_collection = self.database.collection::<User>(USER);
        let user_query = user_collection.update_many(doc! {},
                                                     doc! {"$pull": {"allowances": {"note_id": &note_id}}},
                                                     None).await;
        if user_query.is_err() { return Err(DBError::QueryError(format!("remove references to note with ID='{}'", note_id))) }

        // Remove Note
        let note_collection = self.database.collection::<Note>(NOTES);
        let filter = doc! {"_id": ObjectId::from_str(note_id).unwrap()};

        return match note_collection.delete_one(filter, None).await {
            Ok(_) => Ok(()),
            Err(_) => Err(DBError::QueryError(format!("removal of note with ID='{}'", note_id)))
        }
    }
}