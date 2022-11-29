//! Implementation for tasks regarding a specific note

use std::str::FromStr;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson, Database};
use crate::storage::error::DBError;
use crate::storage::interface::{NoteManager, NoteMeta, PermissionLevel};
use crate::storage::mongodb::{get_note, get_user};
use crate::storage::mongodb::schema::{Note, NOTES, User, USER};

/// A struct caching values a previous note-lookup already included
struct MongoDBNoteCache {
    /// The title of the note
    title: String,
    /// The content of the note
    content: String,
    /// The tags of the note
    tags: Vec<String>
}

/// Implements the [`NoteManager`]-trait
pub(in crate::storage::mongodb) struct MongoDBNoteManager {
    meta: NoteMeta,
    database: Database,
    cache: MongoDBNoteCache // Saves from unnecessary calls to the DB at a (minimal) cost of accuracy
}

impl MongoDBNoteManager {
    pub async fn new(note_id: &str, perm: PermissionLevel, db: &Database) -> Result<MongoDBNoteManager, DBError> {
        let note = get_note(note_id, db).await?;
        Ok(MongoDBNoteManager {
            meta: NoteMeta {
                id: note_id.to_string(),
                permission: perm,
                owner_id: note.owner_id.clone(),
                created_at: DateTime::from(note.created_at),
                last_edited: DateTime::from(note.last_edited)
            },
            database: db.clone(),
            cache: MongoDBNoteCache {
                title: note.title,
                content: note.content,
                tags: note.tags
            }
        })
    }
}

#[async_trait]
impl NoteManager for MongoDBNoteManager {
    fn get_meta_information(&self) -> &NoteMeta {
        &self.meta
    }


    async fn get_title(&self) -> Result<String, DBError> {
        //Ok(get_note(&self.meta.id, &self.database).await?.title)
        Ok(self.cache.title.clone())
    }


    async fn set_title(&mut self, title: String) -> Result<(), DBError> {
        self.check_permission(PermissionLevel::ReadWrite)?;
        let note_collection = self.database.collection::<Note>(NOTES);
        let filter = doc! { "_id": ObjectId::from_str(&self.meta.id).unwrap() };
        let stamp = Utc::now();

        let query = doc! { "$set": { "title": &title, "last_edited": &stamp } };
        let query_response = note_collection.update_one(filter, query, None).await;
        return if query_response.is_err() {
            Err(DBError::QueryError(format!("set title of note with ID='{}' to '{}'", self.get_meta_information().id, title)))
        } else {
            // Update meta-struct
            self.meta.last_edited = stamp;
            // Update cache
            self.cache.title = title;
            Ok(())
        }
    }


    async fn get_content(&self) -> Result<String, DBError> {
        //Ok(get_note(&self.meta.id, &self.database).await?.content)
        Ok(self.cache.content.clone())
    }

    async fn set_content(&mut self, content: String) -> Result<(), DBError> {
        self.check_permission(PermissionLevel::ReadWrite)?;
        let note_collection = self.database.collection::<Note>(NOTES);
        let filter = doc! { "_id": ObjectId::from_str(&self.meta.id).unwrap() };
        let stamp = Utc::now();

        let query = doc! { "$set": { "content": &content, "last_edited": &stamp } };
        let query_response = note_collection.update_one(filter, query, None).await;
        return if query_response.is_err() {
            Err(DBError::QueryError(format!("set content of note with ID='{}'", self.get_meta_information().id)))
        } else {
            // Update meta-struct
            self.meta.last_edited = stamp;
            // Update cache
            self.cache.content = content;
            Ok(())
        }
    }


    async fn get_tags(&self) -> Result<Vec<String>, DBError> {
        //Ok(get_note(&self.meta.id, &self.database).await?.tags)
        Ok(self.cache.tags.clone())
    }

    async fn set_tags(&mut self, tags: Vec<String>) -> Result<(), DBError> {
        self.check_permission(PermissionLevel::ReadWrite)?;
        let note_collection = self.database.collection::<Note>(NOTES);
        let filter = doc! { "_id": ObjectId::from_str(&self.meta.id).unwrap() };
        let stamp = Utc::now();

        let query = doc! { "$set": { "tags": &tags, "last_edited": &stamp } };
        let query_response = note_collection.update_one(filter, query, None).await;
        return if query_response.is_err() {
            Err(DBError::QueryError(format!("set tags of note with ID='{}'", self.get_meta_information().id)))
        } else {
            // Update meta-struct
            self.meta.last_edited = stamp;
            // Update cache
            self.cache.tags = tags;
            Ok(())
        }
    }


    async fn update_share(&self, user_id: &str, perm: PermissionLevel) -> Result<(), DBError> {
        self.check_permission(PermissionLevel::Moderate)?;

        // Check for association with the owner
        let user = get_user(user_id, &self.database).await?;
        if !user.connections.contains(&self.meta.owner_id) {
            return Err(DBError::InvalidRequestError(format!("'{}' and '{}' don't associate with each other", self.meta.owner_id, user_id)))
        }

        // Update allowance
        let allowance = user.allowances.iter()
            .find(|allow| allow.note_id.eq(&self.meta.id));
        let user_collection = self.database.collection::<User>(USER);
        let mut filter = doc! {"_id": &user_id};

        let query = if perm.eq(&PermissionLevel::Forbidden) {
            if allowance.is_none() { return Err(DBError::InvalidRequestError(format!("'{}' has no permission to revoke", user_id))) }
            // Remove the allowance
            doc! {"$pull": {"allowances": {"note_id": &self.meta.id}}}
        } else {
            // Add or update the allowance
            if allowance.is_none() {
                doc! {
                    "$push": {
                        "allowances": {
                            "note_id": &self.meta.id,
                            "level": bson::to_bson(&perm).unwrap(),
                            "owner_id": &self.meta.owner_id
                        }
                    }
                }
            } else {
                filter = doc! {"_id": &user_id, "allowances.note_id": &self.meta.id};
                doc! {"$set": {"allowances.$.level": bson::to_bson(&perm).unwrap()}}
            }
        };
        let query_response = user_collection.update_one(filter, query, None).await;

        return if query_response.is_ok() { Ok(()) } else {
            Err(DBError::QueryError(format!("add/update user's(ID='{}') share of note(ID='{}')", user_id, self.get_meta_information().id)))
        }
    }
}