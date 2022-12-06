//! Endpoints regarding note-objects and their manipulation

use actix_web::{get, put, delete, post, Responder, HttpResponse, web::Path, web};
use mongodb::bson::doc;
use crate::storage::error::DBError;
use crate::storage::interface::UserManager;
use crate::web::error::APIError;
use crate::web::note::json_objects::{NoteRequest, NoteResponse};
use crate::web::{ResponseObject, ResponseObjectWithPayload};

// Response-/Request-Objects
/// Structs modelling the request- and response-bodies
mod json_objects {
    use serde::{Serialize, Deserialize};
    use crate::storage::interface::{NoteManager, PermissionLevel};

    /// Body of a request containing a note
    #[derive(Deserialize)]
    pub struct NoteRequest {
        /// Title of the note
        pub title: String,
        /// Content of the note
        pub content: String,
        /// Tags associated with the note
        pub tags: Vec<String>
    }

    /// Body of a response containing a note
    #[derive(Serialize)]
    pub struct NoteResponse {
        /// The identifier of the note
        pub note_id: String,
        /// The note-object
        pub note: Note,
        /// The level of access the requesting user has regarding the note
        pub allowance: PermissionLevel
    }

    impl NoteResponse {
        pub async fn from(note_manager: &Box<dyn NoteManager>) -> NoteResponse { //TODO Catch error
            NoteResponse {
                note_id: note_manager.get_meta_information().id.clone(),
                note: Note {
                    title: note_manager.get_title().await.unwrap(),
                    content: note_manager.get_content().await.unwrap(),
                    owner_id: note_manager.get_meta_information().owner_id.clone(),
                    tags: note_manager.get_tags().await.unwrap()
                },
                allowance: note_manager.get_meta_information().permission}
        }
    }

    //TODO REMOVE
    /// A struct modelling a note
    #[derive(Debug, Serialize)]
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
}

/// ENDPOINT: Takes a note and inserts it into the database
///
/// Returns one of the following HttpResponses:
/// * `201`
///     - \[Body: JSON\] Note was inserted successfully
/// * `401`
///     - **\[10\]** Missing or invalid JWT
/// * `500`
///     - Something went wrong internally (debug)
///
/// # Arguments
///
/// * `note_req` - The body of the request parsed to a NoteRequest-object
/// * `user_manager` - A [`UserManager`]-instance of the requesting user
///
/// # Examples
///
/// ```text
/// POST-Request at `{api-url}/note` with a cookie containing a valid JWT
///     {
///         "title": "Test-Note",
///         "content": "This is but a simple demonstration",
///         "tags": ["Test", "Note"]
///     }
/// => 201
///     {
///         "success": true,
///         "content": {
///             "note_id": "7254fa970b62u3ag62dr4d3l",
///             "note": {
///                 "title": "Test-Note",
///                 "content": "This is but a simple demonstration",
///                 "owner_id": "testUser",
///                 "tags": [
///                     "Test",
///                     "Note"
///                 ]
///             },
///             "allowance": "Owner"
///         },
///         "time": "2022-04-11 12:20:28"
///     }
/// ```
/// ```text
/// POST-Request at `{api-url}/note` without a cookie containing a JWT
///     {
///         "title": "Test-Note",
///         "content": "This is but a simple demonstration",
///         "tags": ["Test", "Note"]
///     }
/// => 401
///     {
///         "success": false,
///         "code": 10,
///         "message": "user is not logged in",
///         "time": "2022-04-11 12:20:19"
///     }
/// ```
#[post("/note")]
pub async fn add_note(note_req: web::Json<NoteRequest>, user_manager: Box<dyn UserManager>) -> impl Responder {
    let note_req = note_req.into_inner();
    // Add the new note to the db
    match user_manager.add_note(&note_req.title).await {
        Ok(mut note_manager) => {
            // Add content if specified
            if !note_req.content.is_empty() && note_manager.set_content(note_req.content).await.is_err() {
                return Err(APIError::QueryError("failed to set content of note".to_string())) }
            // Add tags if specified
            if !note_req.tags.is_empty() && note_manager.set_tags(note_req.tags).await.is_err() {
                return Err(APIError::QueryError("failed to set tags of note".to_string())) }
            Ok(HttpResponse::Created().json(
                ResponseObjectWithPayload::new(
                    NoteResponse::from(&note_manager).await
                )
            ))
        },
        Err(_) => Err(APIError::QueryError("note could not be saved to db".to_string())) //unknown //TODO Catch various errors
    }
}

/// ENDPOINT: Returns a note from the database using it's identifier
///
/// Returns one of the following HttpResponses:
/// * `200`
///     - Note can be returned
/// * `400`
///     - **\[21\]** id contains invalid symbols
/// * `401`
///     - **\[10\]** Missing or invalid JWT
/// * `403`
///     - **\[12\]** Insufficient access-level (no read-access)
/// * `500`
///     - Something went wrong internally (debug)
///
/// # Arguments
///
/// * `path` - A Path-object containing the id of the to-be-returned note
/// * `user_manager` - A [`UserManager`]-instance of the requesting user
///
/// # Examples
///
/// ```text
/// GET-Request at `{api-url}/note/7254fa970b62u3ag62dr4d3l` with a cookie containing a valid JWT
/// => 200
///     {
///         "success": true,
///         "content": {
///             "note_id": "7254fa970b62u3ag62dr4d3l",
///             "note": {
///                 "title": "Test-Note",
///                 "content": "This is but a simple demonstration",
///                 "owner_id": "testUser",
///                 "tags": [
///                     "Test",
///                     "Note"
///                 ]
///             },
///             "allowance": "Owner"
///         },
///         "time": "2022-04-11 12:20:28"
///     }
/// ```
/// ```text
/// GET-Request at `{api-url}/note/72}4fa97$b62u3:2dr4d3l` without a cookie containing a JWT
/// => 400
///     {
///         "success": false,
///         "code": 21,
///         "message": "requested id contains forbidden character",
///         "time": "2022-04-11 12:20:19"
///     }
/// ```
/// ```text
/// GET-Request at `{api-url}/note/7254fa970b62u3ag62dr4d3l` without a cookie containing a JWT
/// => 401
///     {
///         "success": false,
///         "code": 10,
///         "message": "user is not logged in",
///         "time": "2022-04-11 12:20:19"
///     }
/// ```
/// ```text
/// GET-Request at `{api-url}/note/7254fa970b62u3ag62dr4d3l` to a note the current user is not allowed to read
/// => 403
///     {
///         "success": false,
///         "code": 12,
///         "message": "no permission",
///         "time": "2022-04-11 12:20:19"
///     }
/// ```
#[get("/note/{note_id}")]
pub async fn get_note(path: Path<String>, user_manager: Box<dyn UserManager>) -> impl Responder {
    let note_id = path.into_inner();

    // Attempt to get the note
    match user_manager.get_note(&note_id).await {
        Ok(note_manager) => Ok(HttpResponse::Ok().json(
            ResponseObjectWithPayload::new(
                NoteResponse::from(&note_manager).await
            )
        )),
        Err(DBError::NoPermissionError) => Err(APIError::NoPermissionError),
        Err(DBError::InvalidSequenceError(_)) => Err(APIError::InvalidIDError),
        Err(_) => Err(APIError::QueryError("failed to retrieve note".to_string()))
    }
}

/// ENDPOINT: Takes a note and updates its counterpart in the database with its own values
///
/// Returns one of the following HttpResponses:
/// * `200`
///     - \[Body: JSON\] Note was updated successfully
/// * `400`
///     - **\[21\]** id contains invalid symbols
/// * `401`
///     - **\[10\]** Missing or invalid JWT
/// * `403`
///     - **\[12\]** Insufficient access-level (no write-access)
/// * `500`
///     - Something went wrong internally (debug)
///
/// # Arguments
///
/// * `path` - A Path-object containing the id of the to-be-deleted note
/// * `note_req` - The body of the request parsed to a NoteRequest-object
/// * `user_manager` - A [`UserManager`]-instance of the requesting user
///
/// # Examples
///
/// ```text
/// PUT-Request at `{api-url}/note/7254fa970b62u3ag62dr4d3l` with a cookie containing a valid JWT
///     {
///         "title": "Test-Note",
///         "content": "This is but a simple demonstration",
///         "tags": ["Test", "Note", "Updated"]
///     }
/// => 200
///     {
///         "success": true,
///         "content": {
///             "note_id": "7254fa970b62u3ag62dr4d3l",
///             "note": {
///                 "title": "Test-Note",
///                 "content": "This is but a simple demonstration",
///                 "owner_id": "testUser",
///                 "tags": [
///                     "Test",
///                     "Note",
///                     "Updated"
///                 ]
///             },
///             "allowance": "Owner"
///         },
///         "time": "2022-04-11 12:20:28"
///     }
/// ```
/// ```text
/// PUT-Request at `{api-url}/note/72}4fa97$b62u3:2dr4d3l` without a cookie containing a JWT
///     {
///         "title": "Test-Note",
///         "content": "This is but a simple demonstration",
///         "tags": ["Test", "Note", "Updated"]
///     }
/// => 400
///     {
///         "success": false,
///         "code": 21,
///         "message": "requested id contains forbidden character",
///         "time": "2022-04-11 12:20:19"
///     }
/// ```
/// ```text
/// PUT-Request at `{api-url}/note/7254fa970b62u3ag62dr4d3l` without a cookie containing a JWT
///     {
///         "title": "Test-Note",
///         "content": "This is but a simple demonstration",
///         "tags": ["Test", "Note", "Updated"]
///     }
/// => 401
///     {
///         "success": false,
///         "code": 10,
///         "message": "user is not logged in",
///         "time": "2022-04-11 12:20:19"
///     }
/// ```
/// ```text
/// PUT-Request at `{api-url}/note/7254fa970b62u3ag62dr4d3l` to a note the current user is not allowed to write to
///     {
///         "title": "Test-Note",
///         "content": "This is but a simple demonstration",
///         "tags": ["Test", "Note", "Updated"]
///     }
/// => 403
///     {
///         "success": false,
///         "code": 12,
///         "message": "no permission",
///         "time": "2022-04-11 12:20:19"
///     }
/// ```
#[put("/note/{note_id}")]
pub async fn update_note(path: Path<String>, note_req: web::Json<NoteRequest>, user_manager: Box<dyn UserManager>) -> impl Responder {
    let note_req = note_req.into_inner();
    let note_id = path.into_inner();

    // Attempt to get the note
    match user_manager.get_note(&note_id).await {
        Ok(mut note_manager) => {
            // Update the individual fields of the note
            let mut update_results = Vec::new();
            update_results.push(note_manager.set_title(note_req.title).await);
            update_results.push(note_manager.set_content(note_req.content).await);
            update_results.push(note_manager.set_tags(note_req.tags).await);

            // Verify that all changes got through
            for result in update_results {
                match result {
                    Ok(_) => continue,
                    Err(DBError::NoPermissionError) => return Err(APIError::NoPermissionError),
                    Err(_) => return Err(APIError::QueryError("update of note failed".to_string()))
                }
            }

            // Return success-response
            Ok(HttpResponse::Ok().json(
                ResponseObjectWithPayload::new(
                    NoteResponse::from(&note_manager).await
                )
            ))
        },
        Err(DBError::NoPermissionError) => Err(APIError::NoPermissionError),
        Err(DBError::InvalidSequenceError(_)) => Err(APIError::InvalidIDError),
        Err(_) => Err(APIError::QueryError("failed to retrieve note".to_string()))
    }
}

/// ENDPOINT: Removes a note from the database using it's identifier
///
/// Returns one of the following HttpResponses:
/// * `200`
///     - Note was removed successfully
/// * `400`
///     - **\[21\]** id contains invalid symbols
/// * `401`
///     - **\[10\]** Missing or invalid JWT
/// * `403`
///     - **\[12\]** Insufficient access-level (not owner)
/// * `500`
///     - Something went wrong internally (debug)
///
/// # Arguments
///
/// * `path` - A Path-object containing the id of the to-be-deleted note
/// * `user_manager` - A [`UserManager`]-instance of the requesting user
///
/// # Examples
///
/// ```text
/// DELETE-Request at `{api-url}/note/7254fa970b62u3ag62dr4d3l` with a cookie containing a valid JWT
/// => 200
///     {
///         "success": true,
///         "time": "2022-04-11 12:20:19"
///     }
/// ```
/// ```text
/// DELETE-Request at `{api-url}/note/72}4fa97$b62u3:2dr4d3l` with a cookie containing a valid JWT
/// => 400
///     {
///         "success": false,
///         "code": 21,
///         "message": "requested id contains forbidden character",
///         "time": "2022-04-11 12:20:19"
///     }
/// ```
/// ```text
/// DELETE-Request at `{api-url}/note/7254fa970b62u3ag62dr4d3l` without a cookie containing a JWT
/// => 401
///     {
///         "success": false,
///         "code": 10,
///         "message": "user is not logged in",
///         "time": "2022-04-11 12:20:19"
///     }
/// ```
/// ```text
/// DELETE-Request at `{api-url}/note/7254fa970b62u3ag62dr4d3l` to a note the current user is not allowed to delete
/// => 403
///     {
///         "success": false,
///         "code": 12,
///         "message": "no permission",
///         "time": "2022-04-11 12:20:19"
///     }
/// ```
#[delete("/note/{note_id}")]
pub async fn remove_note(path: Path<String>, user_manager: Box<dyn UserManager>) -> impl Responder {
    let note_id = path.into_inner();

    match user_manager.remove_note(&note_id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ResponseObject::new())),
        Err(DBError::NoPermissionError) => Err(APIError::NoPermissionError),
        Err(DBError::InvalidSequenceError(_)) => Err(APIError::InvalidIDError),
        Err(DBError::MissingEntryError(_,_)) => Err(APIError::NoPermissionError), // note doesnt exist
        Err(_) => Err(APIError::QueryError("failed to remove note".to_string()))
    }
}
