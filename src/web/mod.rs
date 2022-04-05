//! REST-Endpoints and their logic
//!
//! # Endpoints:
//!
//! + System:
//!     * `GET /system`             - Get System Information [[`return_system_status`]]
//!
//! + Authorisation:
//!     * `POST /auth`              - Login [[`authenticate`](auth::authenticate)]
//!     * `DELETE /auth`            - Logout [[`logout`](auth::logout)]
//!
//! + Notes:
//!     * `GET /notes`              - List of all available notes [[`list_notes`]]
//!
//!     * `POST /note`              - Add a note [[`add_note`](note::add_note)]
//!     * `GET /note/{note_id}`     - Get a note [[`get_note`](note::get_note)]
//!     * `PUT /note/{note_id}`     - Update a note [[`update_note`](note::update_note)]
//!     * `DELETE /note/{note_id}`  - Remove a note [[`remove_note`](note::remove_note)]
//!
//! + User:
//!     * `POST /user`              - Create a new user [[`add_user`](user::add_user)]
//!     * `GET /user`               - Get current user [[`get_user`](user::get_user)]
//!     * `DELETE /user`            - Delete the current user and logout [[`remove_user`](user::remove_user)]
//!
//! + Shares:
//!     * `GET /share`              - Generate an invite code [[`get_relation_code`](share::get_relation_code)]
//!     * `POST /share`             - Use an invite code to create a relation between two user [[`create_relation`](share::create_relation)]
//!     * `DELETE /share/{user_id}` - Remove the relation between two user [[`remove_relation`](share::remove_relation)]
//!     * `PUT /share/{note_id}`    - Update other users access-rights regarding the note [[`update_allowances`](share::update_allowances)]

mod note;
mod user;
mod share;
mod error;
mod auth;

use std::env;
use std::sync::Mutex;
use serde::Serialize;
use actix_web::{get, HttpRequest, HttpResponse, Responder, web::{ServiceConfig, Data}};
use mongodb::{bson::doc, Database};
use crate::db_access::{AllowanceLevel, DBError, get_dbo_by_id, Note, NOTES};
use crate::web::auth::get_user_from_request;
use crate::web::error::AuthError;

/// Configures the web-server to add all endpoints
///
/// # Arguments
///
/// * `cfg` - A mutable ServiceConfig which takes the endpoints
///
/// # Examples
///
/// ```
/// mod web;
/// use actix_web::{App, HttpServer};
///
/// #[actix_rt::main]
/// async fn main() -> std::io::Result<()> {
///     HttpServer::new(move ||
///         App::new()
///             .service(actix_web::web::scope("/api").configure(web::handler_config)))
///         .bind(("127.0.0.1", 8080))?
///         .run()
///         .await
/// }
/// ```
pub fn handler_config(cfg: &mut ServiceConfig) {
    // Add all special handler
    cfg.service(return_system_status)
        .service(auth::authenticate)
        .service(list_notes)
        .service(auth::logout);
    // Add all note-related handler
    cfg.service(note::add_note)
        .service(note::get_note)
        .service(note::update_note)
        .service(note::remove_note);
    // Add all user-related handler
    cfg.service(user::add_user)
        .service(user::get_user)
        .service(user::remove_user);
    // Add all share-related handler
    cfg.service(share::get_relation_code)
        .service(share::create_relation)
        .service(share::remove_relation)
        .service(share::update_allowances);
}

/// ENDPOINT: Returns information on the system currently running.
///
/// Returns one of the following HttpResponses:
/// * `200` [Body: JSON] - System-information could be compiled
///
/// # Examples
///
/// ```text
/// GET-Request at `{api-url}/system`
/// => 200
///     {
///         "application": "writeUp",
///         "version": "0.1.0"
///     }
/// ```
#[get("/system")]
async fn return_system_status() -> impl Responder { //TODO flesh out
    // Define Response-Object
    /// Response-body containing information on the system currently running
    #[derive(Serialize)]
    struct SystemResponse {
        /// The name of the application currently running
        application: String,
        /// The version of the application currently running
        version: String
    }
    return HttpResponse::Ok().json(SystemResponse {
        application: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string()
    })
}

/// ENDPOINT: Compiles a list of all notes the current user has access to.
///
/// Returns one of the following HttpResponses:
/// * `200` [Body: JSON] - List could be compiled
/// * `401` - No user could be verified
/// * `500` - Something went wrong internally (debug)
///
/// # Arguments
///
/// * `req` - The HttpRequest that was made
/// * `db` - The AppData containing a Mutex-secured Database-connection
///
/// # Examples
///
/// ```text
/// GET-Request at `{api-url}/notes` with a cookie containing a valid JWT
/// => 200
///     [
///         {
///             "note_id": "7254fa970b62u3ag62dr4d3l",
///             "title": "Test-Note",
///             "tags": [
///                 "Test",
///                 "Note"
///             ],
///             "allowance": "Owner"
///         },
///         {
///             "note_id": "7354fa9uu782u3ag62t54d3l",
///             "title": "Note, but this time different",
///             "tags": [
///                 "Note",
///                 "Different"
///             ],
///             "allowance": "Read"
///         }
///     ]
/// ```
/// ```text
/// GET-Request at `{api-url}/notes` without a cookie containing a JWT
/// => 401
///     "token-cookie was not found"
/// ```
#[get("/notes")]
async fn list_notes(req: HttpRequest, db: Data<Mutex<Database>>) -> impl Responder {
    // Define Response-Object
    /// Response-body containing a limited amount of information on a note
    #[derive(Serialize)]
    struct ReducedNoteResponse {
        /// The note's identifier
        note_id: String,
        /// The note's title
        title: String,
        /// The tags associated with the note
        tags: Vec<String>,
        /// The level of access the current user has regarding this note
        allowance: AllowanceLevel
    }
    match get_user_from_request(req, db.get_ref()).await {
        Ok(user) => {
            let mut response_vector = Vec::new();
            for allowance in user.allowances {
                // Read all allowed notes and create response-objects
                match get_dbo_by_id::<Note>(NOTES, allowance.note_id.clone(), db.get_ref()).await {
                    Ok(note) => response_vector.push(ReducedNoteResponse {
                        note_id: allowance.note_id,
                        title: note.title,
                        tags: note.tags,
                        allowance: allowance.level
                    }),
                    Err(DBError::NoDocumentFoundError) => return AuthError::InternalServerError(
                        format!("reference to deleted note(ID:{}) still exists in user-allowances",
                                allowance.note_id)).gen_response(), //user has allowance for a nonexisting note
                    Err(_) => {} //unknown
                }
            }
            // Return the compiled list of notes
            HttpResponse::Ok().json(response_vector)
        }
        Err(e) => e.gen_response()
    }
}
