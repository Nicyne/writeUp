//! Endpoints regarding the sharing of notes and connecting of users

use std::sync::Mutex;
use actix_web::{get, put, post, delete, Responder, HttpRequest, HttpResponse, web};
use actix_web::web::{Data, Path};
use chrono::Utc;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use mongodb::bson::doc;
use serde::{Serialize, Deserialize};
use mongodb::{bson, Database};
use crate::db_access::{update_dbo_by_id, USER, User, filter_allowances_by_user_id, AllowanceLevel, get_dbo_by_id};
use crate::db_access::{AllowanceLevel::Forbidden, DBError::QueryError};
use crate::web::{auth::get_user_from_request, note::get_allow_level_for_note};
use crate::web::error::{AuthError, AuthError::InternalServerError};
use crate::web::share::json_objects::{InviteBody, ShareRequest, SuccessResponse};

// Invite-Assets
/// Secret-phrase used to en- and decode invite-codes
const INVITE_SECRET: &[u8] = env!("SHARE_SECRET").as_bytes(); //TODO secret is static
/// Time in minutes until an invite expires
const INVITE_DURATION_MINUTES: i64 = 30;

/// Struct containing all information to be encoded in the Invitation
#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    /// Username of the inviting user
    sub: String,
    /// Timestamp of invite-expiration
    exp: usize,
}

// Response-/Request-Objects
/// Structs modelling the request- and response-bodies
mod json_objects {
    use serde::{Serialize, Deserialize};
    use crate::db_access::AllowanceLevel;

    /// Body of both request and response containing an invite-code
    #[derive(Serialize, Deserialize)]
    pub struct InviteBody {
        /// An invite-code
        pub code: String
    }

    /// Body of a request for new or altered Allowances
    #[derive(Deserialize)]
    pub struct ShareRequest {
        /// User a note is to be shared with
        pub user_id: String,
        /// The level of access being given to the user
        pub allowance: AllowanceLevel
    }

    #[derive(Serialize)]
    pub struct SuccessResponse { //TODO remove and replace with actual responses
        pub success: bool
    }
}

/// ENDPOINT: Creates an invitation code to allow the connection of two user
///
/// Returns one of the following HttpResponses:
/// * `200` - Code has been created and is being returned
/// * `401` - Missing or invalid JWT
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
/// GET-Request at `{api-url}/share` with a cookie containing a valid JWT
/// => 200
///     {
///         "code": "opH6eXAbVbJFR3QiDFQhbGciOiGTUzI1NiJ9.eyJggRLiOiJ5RLKM2IiwiZXhwIjoxNjQ4NzE5MTEzfQ.CUIReWW7JAj8q7cnJx93ofcsyrWfJh5VLJAj57vEwe4Q"
///     }
/// ```
/// ```text
/// GET-Request at `{api-url}/share` without a cookie containing a JWT
/// => 401
///     "token-cookie was not found"
/// ```
#[get("/share")]
pub async fn get_relation_code(req: HttpRequest, db:Data<Mutex<Database>>) -> impl Responder {
    match get_user_from_request(req, &db).await {
        Ok(user) => {
            match gen_invite(&user._id) {
                Ok(code) => HttpResponse::Ok().json(InviteBody {code}),
                Err(e) => e.gen_response()
            }
        }
        Err(e) => e.gen_response()
    }
}

/// ENDPOINT: Attempts to create a relation between two user with an invite-code
///
/// Returns one of the following HttpResponses:
/// * `200` [Body: JSON] - Relation could be established
/// * `401` - Missing or invalid JWT / Invalid invitation-code
/// * `500` - Something went wrong internally (debug)
///
/// # Arguments
///
/// * `req` - The HttpRequest that was made
/// * `code_req` - The body of the request parsed to an InviteBody-object
/// * `db` - The AppData containing a Mutex-secured Database-connection
///
/// # Examples
///
/// ```text
/// POST-Request at `{api-url}/share` with a cookie containing a valid JWT
///     {
///         "code": "opH6eXAbVbJFR3QiDFQhbGciOiGTUzI1NiJ9.eyJggRLiOiJ5RLKM2IiwiZXhwIjoxNjQ4NzE5MTEzfQ.CUIReWW7JAj8q7cnJx93ofcsyrWfJh5VLJAj57vEwe4Q"
///     }
/// => 200
///     {
///         "success": true
///     }
/// ```
/// ```text
/// POST-Request at `{api-url}/share` without a cookie containing a JWT
///     {
///         "code": "opH6eXAbVbJFR3QiDFQhbGciOiGTUzI1NiJ9.eyJggRLiOiJ5RLKM2IiwiZXhwIjoxNjQ4NzE5MTEzfQ.CUIReWW7JAj8q7cnJx93ofcsyrWfJh5VLJAj57vEwe4Q"
///     }
/// => 401
///     "token-cookie was not found"
/// ```
/// ```text
/// POST-Request at `{api-url}/share` with a cookie containing a valid JWT [invite-code is expired]
///     {
///         "code": "opH6eXAbVbJFR3QiDFQhbGciOiGTUzI1NiJ9.eyJggRLiOiJ5RLKM2IiwiZXhwIjoxNjQ4NzE5MTEzfQ.CUIReWW7JAj8q7cnJx93ofcsyrWfJh5VLJAj57vEwe4Q"
///     }
/// => 401
///     "token-cookie was not found"
/// ```
#[post("/share")]
pub async fn create_relation(req: HttpRequest, code_req: web::Json<InviteBody>, db: Data<Mutex<Database>>) -> impl Responder {
    match get_user_from_request(req, &db).await {
        Ok(user) => {
            match get_user_id_from_invite_code(&code_req.code) {
                Ok(invite_user_id) => {
                    // Simple (non exhaustive) check for an already existing connection between users
                    if invite_user_id.eq(&user._id) || user.connections.contains(&invite_user_id) {
                        return InternalServerError("User already share a connection".to_string()).gen_response() //TODO Review error-types
                    }

                    // Add each user to the others relation-list
                    let update_curr_user =
                        update_dbo_by_id::<User>(USER, user._id.clone(),
                                                 doc! {"$push": {"connections": &invite_user_id}},
                                                 &db);
                    let update_invite_user =
                        update_dbo_by_id::<User>(USER, invite_user_id,
                                                 doc! {"$push": {"connections": user._id}},
                                                 &db);

                    // Wait for the queries to finish and check for an error
                    if update_curr_user.await.is_err() || update_invite_user.await.is_err() {
                        return InternalServerError("Relation could not be established".to_string()).gen_response()
                    }
                    HttpResponse::Ok().json(SuccessResponse { success: true })
                }
                Err(e) => e.gen_response()
            }
        }
        Err(e) => e.gen_response()
    }
}

/// ENDPOINT: Removes a relation between two user
///
/// Returns one of the following HttpResponses:
/// * `200` - Relation has been successfully removed
/// * `401` - Missing or invalid JWT
/// * `500` - Something went wrong internally (debug)
///
/// # Arguments
///
/// * `path` - A Path-object containing the id of the related user
/// * `req` - The HttpRequest that was made
/// * `db` - The AppData containing a Mutex-secured Database-connection
///
/// # Examples
///
/// ```text
/// DELETE-Request at `{api-url}/share/testUser` with a cookie containing a valid JWT
/// => 200
///     {
///         "success": true
///     }
/// ```
/// ```text
/// DELETE-Request at `{api-url}/share/testUser` without a cookie containing a JWT
/// => 401
///     {
///         "error": "token-cookie was not found"
///     }
/// ```
#[delete("/share/{user_id}")]
pub async fn remove_relation(path: Path<String>, req: HttpRequest, db: Data<Mutex<Database>>) -> impl Responder {
    let related_user = path.into_inner();
    match get_user_from_request(req, &db).await {
        Ok(user) => {
            // Simple (non exhaustive) check for an already existing connection between users
            if user._id.eq(&related_user) || !user.connections.contains(&related_user) {
                return InternalServerError("User don't share a connection".to_string()).gen_response() //TODO Review error-types
            }

            // Compile all notes that have been shared between both user
            let allow_curr_user =
                filter_allowances_by_user_id(&user._id, &related_user, &db);
            let allow_rel_user =
                filter_allowances_by_user_id(&related_user, &user._id, &db);

            // Remove all allowances to notes of the other host
            let remove_allow_curr_user =
                update_dbo_by_id::<User>(USER,user._id.clone(),
                                         doc! {"$pull": {"allowances": {"note_id": {"$in": allow_curr_user.await.unwrap()}}}},
                                         &db);
            let remove_allow_rel_user =
                update_dbo_by_id::<User>(USER, related_user.clone(),
                                         doc! {"$pull": {"allowances": {"note_id": {"$in": allow_rel_user.await.unwrap()}}}},
                                         &db);
            // Remove the relation from each of the user
            let remove_conn_curr_user =
                update_dbo_by_id::<User>(USER, user._id.clone(),
                                         doc! {"$pull": {"connections": &related_user}},
                                         &db);
            let remove_conn_rel_user =
                update_dbo_by_id::<User>(USER, related_user.clone(),
                                         doc! {"$pull": {"connections": user._id.clone()}},
                                         &db);

            // Sync all tasks and check for an error
            let mut error = Vec::new();
            for query in [remove_allow_curr_user, remove_allow_rel_user, remove_conn_curr_user, remove_conn_rel_user] {
                let result = query.await;
                if result.is_err() {
                    error.push(result)
                }
            }
            if !error.is_empty() {
                return InternalServerError("Incomplete removal of relation and shares".to_string()).gen_response()
            }
            HttpResponse::Ok().json(SuccessResponse { success: true })
        }
        Err(e) => e.gen_response()
    }
}

/// ENDPOINT: Takes a list of allowed users and their allowed level of access and updates them accordingly
///
/// Returns one of the following HttpResponses:
/// * `200` [Body: JSON] - All Shares have been updated
/// * `401` - Missing or invalid JWT
/// * `403` - Insufficient access-level (not owner)
/// * `500` - Something went wrong internally (debug)
///
/// # Arguments
///
/// * `path` - A Path-object containing the id of the to-be-shared note
/// * `req` - The HttpRequest that was made
/// * `allow_req` - The body of the request parsed to a Vector containing ShareRequest-objects
/// * `db` - The AppData containing a Mutex-secured Database-connection
///
/// # Examples
///
/// ```text
/// PUT-Request at `{api-url}/share/7254fa970b62u3ag62dr4d3l` with a cookie containing a valid JWT
///     [
///         {
///             "user_id": "testUser",
///             "allowance": "ReadWrite"
///         },
///         {
///             "user_id": "otherUser",
///             "allowance": "Forbidden"
///         }
///     ]
/// => 200
///     {
///         "success": true
///     }
/// ```
/// ```text
/// PUT-Request at `{api-url}/share/7254fa970b62u3ag62dr4d3l` without a cookie containing a JWT
///     [
///         {
///             "user_id": "testUser",
///             "allowance": "ReadWrite"
///         },
///         {
///             "user_id": "otherUser",
///             "allowance": "Forbidden"
///         }
///     ]
/// => 401
///     {
///         "error": "token-cookie was not found"
///     }
/// ```
/// ```text
/// PUT-Request at `{api-url}/share/7254fa970b62u3ag62dr4d3l` to a note the user is not currently the owner of
///     [
///         {
///             "user_id": "testUser",
///             "allowance": "ReadWrite"
///         },
///         {
///             "user_id": "otherUser",
///             "allowance": "Forbidden"
///         }
///     ]
/// => 403
///     {
///         "error": "no permission"
///     }
/// ```
#[put("/share/{note_id}")]
pub async fn update_allowances(path: Path<String>, req: HttpRequest, allow_req: web::Json<Vec<ShareRequest>>, db: Data<Mutex<Database>>) -> impl Responder {
    let note_id = path.into_inner();
    match get_allow_level_for_note(&note_id, req.clone(), &db).await {
        Ok(AllowanceLevel::Owner) => { // Sharing of a note is only allowed to the owner of said note
            let curr_user = get_user_from_request(req, &db).await.unwrap();
            let mut errors = Vec::new();
            // Iterate through all changes in allowances
            for share in allow_req.into_inner() { //TODO Multithread (check for duplicates in user_id first)
                if !curr_user.connections.contains(&share.user_id) { //TODO? Add to error-report
                    continue // No allowances if the user is not connected to the owner
                }
                match get_dbo_by_id::<User>(USER, share.user_id, &db).await {
                    Ok(user) => {
                        match user.allowances.iter().find(|allow| allow.note_id.eq(&note_id)) {
                            // The user already has an existing allowance for the note
                            Some(_) => {
                                if share.allowance.eq(&Forbidden) { // The allowance is to be revoked
                                    if update_dbo_by_id::<User>(USER, user._id,
                                                                doc! {"$pull": {"allowances": {"note_id": &note_id}}}, &db).await.is_err() {
                                        errors.push(QueryError)
                                    }
                                } else { // The allowance is to be altered
                                    let user_coll = db.lock().unwrap().collection::<User>(USER);
                                    if user_coll.update_one(doc! {"_id": user._id, "allowances.note_id": &note_id},
                                                            doc! {"$set": {"allowances.$.level": bson::to_bson(&share.allowance).unwrap()}},
                                                            None).await.is_err() {
                                        errors.push(QueryError)
                                    }
                                }
                            }
                            // The user has no current allowance with the note
                            None => {
                                if share.allowance.eq(&Forbidden) { //TODO? Add to error-report
                                    continue // Can't revoke an allowance that doesn't exist
                                }
                                if update_dbo_by_id::<User>(USER, user._id,
                                                            doc! {"$push": {"allowances":
                                                                {"note_id": &note_id, "level": bson::to_bson(&share.allowance).unwrap()}}},
                                                            &db).await.is_err() {
                                    errors.push(QueryError)
                                }
                            }
                        }
                    }
                    Err(e) => errors.push(e) // Can't share with nonexisting user
                }
            }
            // Check for a failed query
            if !errors.is_empty() { //TODO Create an error report and return it
                return InternalServerError("Failed to update all allowances".to_string()).gen_response() //TODO Review error-types
            }
            HttpResponse::Ok().json(SuccessResponse { success: true })
        }
        Ok(_) =>  AuthError::NoPermissionError.gen_response(), // Not owner of the note
        Err(e) => e.gen_response() // unknown
    }
}

/// Creates an invite-code with the inviting users name as its payload
///
/// # Arguments
///
/// * `uid` - The username to save
fn gen_invite(uid: &str) -> Result<String, AuthError> {
    // Set all required values
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::minutes(INVITE_DURATION_MINUTES))
        .expect("Not a valid timestamp")
        .timestamp();
    let claims = Claims {
        sub: uid.to_owned(),
        exp: expiration as usize
    };
    let header = Header::new(Algorithm::HS256);

    // Generate the invite
    encode(&header, &claims, &EncodingKey::from_secret(INVITE_SECRET))
        .map_err(|_| AuthError::InternalServerError("invite-code could not be created".to_string()))
}

/// Retrieves the username of the inviting user from the invite-code
///
/// # Arguments
///
/// * `code` - Invite-code to be verified
fn get_user_id_from_invite_code(code: &String) -> Result<String, AuthError> {
    decode::<Claims>(&code,
                     &DecodingKey::from_secret(INVITE_SECRET),
                     &Validation::new(Algorithm::HS256))
        .map(|dec|dec.claims.sub).map_err(|_|AuthError::JWTTokenError) //TODO Review error-types
}
