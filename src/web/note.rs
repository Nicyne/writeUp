//! Endpoints regarding note-objects and their manipulation

use std::sync::Mutex;
use actix_web::{get, put, delete, post, Responder, HttpRequest, HttpResponse, web::{Data, Path}, web};
use mongodb::bson::doc;
use mongodb::Database;
use crate::db_access::{AllowanceLevel, DBError, del_dbo_by_id, get_dbo_by_id, insert_dbo, is_safe, Note, NOTES, update_dbo_by_id, User, USER};
use crate::web::error::APIError;
use crate::web::auth::{get_user_from_request, get_user_id_from_request};
use crate::web::note::json_objects::{NoteRequest, NoteResponse};
use crate::web::{ResponseObject, ResponseObjectWithPayload};

// Response-/Request-Objects
/// Structs modelling the request- and response-bodies
mod json_objects {
    use serde::{Serialize, Deserialize};
    use crate::db_access::{AllowanceLevel, Note};

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
    impl NoteRequest {
        /// Converts the Request-body into an actual Note-object
        ///
        /// # Arguments
        ///
        /// * `owner_id` - The user that owns the note
        pub fn to_note(self, owner_id: &str) -> Note {
            Note { title: self.title, content: self.content, owner_id: owner_id.to_string(), tags: self.tags }
        }
    }

    /// Body of a response containing a note
    #[derive(Serialize)]
    pub struct NoteResponse {
        /// The identifier of the note
        pub note_id: String,
        /// The note-object
        pub note: Note,
        /// The level of access the requesting user has regarding the note
        pub allowance: AllowanceLevel
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
/// * `req` - The HttpRequest that was made
/// * `note_req` - The body of the request parsed to a NoteRequest-object
/// * `db` - The AppData containing a Mutex-secured Database-connection
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
pub async fn add_note(req: HttpRequest, note_req: web::Json<NoteRequest>, db: Data<Mutex<Database>>) -> impl Responder {
    let note_req = note_req.into_inner();
    // Grab the user to add a note to
    match get_user_from_request(req, &db).await {
        Ok(user) => {
            // Add the new note to the db
            let note = note_req.to_note(&user._id);
            match insert_dbo::<Note>(NOTES, &note, db.get_ref()).await {
                Ok(res) => {
                    // Add an allowance to the user
                    let note_id = res.inserted_id.as_object_id().unwrap().to_string();
                    match update_dbo_by_id::<User>(USER, user._id,
                                                   doc! {"$push": {"allowances": {"note_id": &note_id, "level": "Owner"}}},
                                                   db.get_ref()).await {
                        Ok(_res) => HttpResponse::Created() // Return the created note
                            .json(ResponseObjectWithPayload::new(NoteResponse { note_id, note, allowance: AllowanceLevel::Owner})), //TODO? Re-fetch object instead of putting together
                        Err(_) => APIError::QueryError("note could not be linked to user-account".to_string()).gen_response() //unknown
                    }
                }
                Err(_) => APIError::QueryError("note could not be saved to db".to_string()).gen_response() //unknown
            }
        }
        Err(e) => e.gen_response()
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
/// * `req` - The HttpRequest that was made
/// * `db` - The AppData containing a Mutex-secured Database-connection
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
pub async fn get_note(path: Path<String>, req: HttpRequest, db: Data<Mutex<Database>>) -> impl Responder {
    let note_id = path.into_inner();
    // Check for potential injection-attempt
    if !is_safe(&note_id) {
        return APIError::InvalidIDError.gen_response()
    }
    // Check if the user has clearance to view this note
    match get_allow_level_for_note(&note_id, req.clone(), db.get_ref()).await {
        Ok(allowance) => {
            // Get note and return it
            match get_dbo_by_id::<Note>(NOTES, note_id.clone(), db.get_ref()).await {
                Ok(note) => HttpResponse::Ok().json(ResponseObjectWithPayload::new(NoteResponse { note_id, note, allowance})),
                Err(DBError::NoDocumentFoundError) => APIError::DBInconsistencyError(
                    get_user_id_from_request(req).unwrap(), note_id).gen_response(), //user has allowance for a nonexisting note
                Err(_) => APIError::QueryError("failed to retrieve note".to_string()).gen_response() //unknown
            }
        }
        Err(e) => e.gen_response()
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
/// * `req` - The HttpRequest that was made
/// * `note_req` - The body of the request parsed to a NoteRequest-object
/// * `db` - The AppData containing a Mutex-secured Database-connection
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
pub async fn update_note(path: Path<String>, req: HttpRequest, note_req: web::Json<NoteRequest>, db: Data<Mutex<Database>>) -> impl Responder {
    let note_req = note_req.into_inner();
    let note_id = path.into_inner();
    // Check for potential injection-attempt
    if !is_safe(&note_id) {
        return APIError::InvalidIDError.gen_response()
    }
    // Check if the user has the clearance to update the note
    match get_allow_level_for_note(&note_id, req.clone(), &db).await {
        Ok(AllowanceLevel::Read) => APIError::NoPermissionError.gen_response(), //Read-Only Access
        Ok(allowance) => {
            // Update all fields of the note
            match update_dbo_by_id::<Note>(NOTES, note_id.clone(), doc! {"$set": { //TODO? Only update changed fields
                "title": &note_req.title,
                "content": &note_req.content,
                "tags": &note_req.tags
            }}, &db).await {
                Ok(_res) => HttpResponse::Ok().json(ResponseObjectWithPayload::new(NoteResponse { //TODO? Re-fetch object instead of putting together
                    note_id,
                    note: note_req.to_note(&get_user_id_from_request(req).unwrap()),
                    allowance
                })),
                Err(_) => APIError::QueryError("update of note failed".to_string()).gen_response() //unknown
            }
        }
        Err(e) => e.gen_response()
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
/// * `req` - The HttpRequest that was made
/// * `db` - The AppData containing a Mutex-secured Database-connection
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
pub async fn remove_note(path: Path<String>, req: HttpRequest, db: Data<Mutex<Database>>) -> impl Responder {
    let note_id = path.into_inner();
    // Check for potential injection-attempt
    if !is_safe(&note_id) {
        return APIError::InvalidIDError.gen_response()
    }
    // Check if the user has the clearance to deleting the note
    match get_allow_level_for_note(&note_id, req, &db).await {
        Ok(AllowanceLevel::Owner) =>  {
            // Remove all allowances
            let users = db.lock().unwrap().collection::<User>(USER);
            match users.update_many(doc! {}, doc! {"$pull": {"allowances": {"note_id": &note_id}}}, None).await { //TODO Dont queue through all notes
                Ok(_res) => {
                    // Remove note
                    match del_dbo_by_id::<Note>(NOTES, note_id, &db).await {
                        Ok(_res) => HttpResponse::Ok().json(ResponseObject::new()),
                        Err(_) => APIError::QueryError("note-object could not be removed".to_string()).gen_response()
                    }
                }
                Err(_) => APIError::QueryError("not all references could be removed".to_string()).gen_response()
            }
        }
        Ok(_) =>  APIError::NoPermissionError.gen_response(),
        Err(e) => e.gen_response()
    }
}

/// Looks up and returns the level of access the current user has regarding the given note
///
/// # Arguments
///
/// * `note_id` - The identifier of the note in question
/// * `req` - The HttpRequest that was made
/// * `db` - A reference to the Mutex-secured Database-connection
pub async fn get_allow_level_for_note(note_id: &str, req: HttpRequest, db: &Mutex<Database>) -> Result<AllowanceLevel, APIError> {
    // Get the User making the request
    match get_user_from_request(req, db).await {
        Ok(user) => {
            // Check if there is an allowance for this note
            match user.allowances.iter().find(|all| all.note_id.eq(&note_id)) {
                Some(allowance) => Ok(allowance.clone().level),
                None => Err(APIError::NoPermissionError)
            }
        }
        Err(e) => Err(e)
    }
}
