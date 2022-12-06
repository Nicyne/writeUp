//! Endpoints regarding the sharing of notes and connecting of users

use std::env;
use actix_web::{get, put, post, delete, Responder, HttpResponse, web};
use actix_web::web::Path;
use chrono::Utc;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use mongodb::bson::doc;
use serde::{Serialize, Deserialize};
use crate::SHARE_SECRET_ENV_VAR_KEY;
use crate::storage::error::DBError;
use crate::storage::interface::UserManager;
use crate::web::{ResponseObject, ResponseObjectWithPayload};
use crate::web::error::APIError;
use crate::web::share::json_objects::{InviteBody, RelationResponse, ShareRequest};

// Invite-Assets
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
    use crate::storage::interface::PermissionLevel;

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
        pub allowance: PermissionLevel
    }

    /// Body of a response after a relation between two user has been established
    #[derive(Serialize)]
    pub struct RelationResponse {
        /// The identifier of the inviting user
        pub user_id: String
    }
}

/// ENDPOINT: Creates an invitation code to allow the connection of two user
///
/// Returns one of the following HttpResponses:
/// * `200`
///     - Code has been created and is being returned
/// * `401`
///     - **\[10\]** Missing or invalid JWT
/// * `500`
///     - Something went wrong internally (debug)
///
/// # Arguments
///
/// * `user_manager` - A [`UserManager`]-instance of the requesting user
///
/// # Examples
///
/// ```text
/// GET-Request at `{api-url}/share` with a cookie containing a valid JWT
/// => 200
///     {
///         "success": true,
///         "content": {
///             "code": "opH6eXAbVbJFR3QiDFQhbGciOiGTUzI1NiJ9.eyJggRLiOiJ5RLKM2IiwiZXhwIjoxNjQ4NzE5MTEzfQ.CUIReWW7JAj8q7cnJx93ofcsyrWfJh5VLJAj57vEwe4Q"
///         },
///         "time": "2022-04-11 12:20:28"
///     }
/// ```
/// ```text
/// GET-Request at `{api-url}/share` without a cookie containing a JWT
/// => 401
///     {
///         "success": false,
///         "code": 10,
///         "message": "user is not logged in",
///         "time": "2022-04-11 12:20:19"
///     }
/// ```
#[get("/share")]
pub async fn get_relation_code(user_manager: Box<dyn UserManager>) -> Result<impl Responder, APIError> {
        Ok(HttpResponse::Ok().json(
            ResponseObjectWithPayload::new(
                InviteBody {
                    code: gen_invite(&user_manager.get_meta_information().id)?
                }
            )
        ))
}

/// ENDPOINT: Attempts to create a relation between two user with an invite-code
///
/// Returns one of the following HttpResponses:
/// * `200`
///     - \[Body: JSON\] Relation could be established
///     - **\[24\]** Invalid instruction (invite code has been issued by the user, connection already exists)
///     - **\[27\]** Invalid invitation-code
/// * `401`
///     - **\[10\]** Missing or invalid JWT
/// * `500`
///     - Something went wrong internally (debug)
///
/// # Arguments
///
/// * `code_req` - The body of the request parsed to an InviteBody-object
/// * `user_manager` - A [`UserManager`]-instance of the requesting user
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
///         "success": true,
///         "content": {
///             "user_id": "otherUser"
///         },
///         "time": "2022-04-11 12:20:28"
///     }
/// ```
/// ```text
/// POST-Request at `{api-url}/share` with a cookie containing a valid JWT [invite-code was issued by the user]
///     {
///         "code": "opH6eXAbVbJFR3QiDFQhbGciOiGTUzI1NiJ9.eyJggRLiOiJ5RLKM2IiwiZXhwIjoxNjQ4NzE5MTEzfQ.CUIReWW7JAj8q7cnJx93ofcsyrWfJh5VLJAj57vEwe4Q"
///     }
/// => 200
///     {
///         "success": false,
///         "code": 24,
///         "message": "invalid instruction: user can't connect with themselves",
///         "time": "2022-04-11 12:20:19"
///     }
/// ```
/// ```text
/// POST-Request at `{api-url}/share` with a cookie containing a valid JWT [invite-code is expired]
///     {
///         "code": "opH6eXAbVbJFR3QiDFQhbGciOiGTUzI1NiJ9.eyJggRLiOiJ5RLKM2IiwiZXhwIjoxNjQ4NzE5MTEzfQ.CUIReWW7JAj8q7cnJx93ofcsyrWfJh5VLJAj57vEwe4Q"
///     }
/// => 200
///     {
///         "success": false,
///         "code": 27,
///         "message": "invite is not valid (anymore)",
///         "time": "2022-04-11 12:20:19"
///     }
/// ```
/// ```text
/// POST-Request at `{api-url}/share` without a cookie containing a JWT
///     {
///         "code": "opH6eXAbVbJFR3QiDFQhbGciOiGTUzI1NiJ9.eyJggRLiOiJ5RLKM2IiwiZXhwIjoxNjQ4NzE5MTEzfQ.CUIReWW7JAj8q7cnJx93ofcsyrWfJh5VLJAj57vEwe4Q"
///     }
/// => 401
///     {
///         "success": false,
///         "code": 10,
///         "message": "user is not logged in",
///         "time": "2022-04-11 12:20:19"
///     }
/// ```
#[post("/share")]
pub async fn create_relation(code_req: web::Json<InviteBody>, user_manager: Box<dyn UserManager>) -> impl Responder {
    let invite_user_id = get_user_id_from_invite_code(&code_req.code)?;

    match user_manager.associate_with(&invite_user_id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(
            ResponseObjectWithPayload::new(
                RelationResponse {
                    user_id: invite_user_id
                }
            )
        )),
        Err(DBError::InvalidRequestError(str)) => Err(APIError::InvalidInstructionsError(str)),
        Err(DBError::InvalidSequenceError(_)) => Err(APIError::InvalidIDError),
        Err(_) => Err(APIError::QueryError("relation could not be established".to_string()))
    }
}

/// ENDPOINT: Removes a relation between two user
///
/// Returns one of the following HttpResponses:
/// * `200`
///     - Relation has been successfully removed
///     - **\[24\]** Invalid instruction (user tries to sever connection to themselves, connection doesn't exist)
/// * `400`
///     - **\[21\]** id contains invalid symbols
/// * `401`
///     - **\[10\]** Missing or invalid JWT
/// * `500`
///     - Something went wrong internally (debug)
///
/// # Arguments
///
/// * `path` - A Path-object containing the id of the related user
/// * `user_manager` - A [`UserManager`]-instance of the requesting user
///
/// # Examples
///
/// ```text
/// DELETE-Request at `{api-url}/share/testUser` with a cookie containing a valid JWT
/// => 200
///     {
///         "success": true,
///         "time": "2022-04-11 12:05:57"
///     }
/// ```
/// ```text
/// DELETE-Request at `{api-url}/share/testUser` with a cookie containing a valid JWT [testUser is not connected to this user]
/// => 200
///     {
///         "success": false,
///         "code": 24,
///         "message": "invalid instruction: user don't share a connection",
///         "time": "2022-04-11 12:20:19"
///     }
/// ```
/// ```text
/// DELETE-Request at `{api-url}/share/te}$t:User` with a cookie containing a JWT
/// => 400
///     {
///         "success": false,
///         "code": 21,
///         "message": "requested id contains forbidden character",
///         "time": "2022-04-11 12:20:19"
///     }
/// ```
/// ```text
/// DELETE-Request at `{api-url}/share/testUser` without a cookie containing a JWT
/// => 401
///     {
///         "success": false,
///         "code": 10,
///         "message": "user is not logged in",
///         "time": "2022-04-11 12:20:19"
///     }
/// ```
#[delete("/share/{user_id}")]
pub async fn remove_relation(path: Path<String>, user_manager: Box<dyn UserManager>) -> impl Responder {
    match user_manager.revoke_association(&path.into_inner()).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ResponseObject::new())),
        Err(DBError::InvalidSequenceError(_)) => Err(APIError::InvalidIDError),
        Err(DBError::InvalidRequestError(msg)) => Err(APIError::InvalidInstructionsError(msg)),
        Err(_) => Err(APIError::QueryError("failed to remove relation".to_string()))
    }
}

/// ENDPOINT: Takes a list of allowed users and their allowed level of access and updates them accordingly
///
/// Returns one of the following HttpResponses:
/// * `200`
///     - \[Body: JSON\] All Shares have been updated
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
/// * `path` - A Path-object containing the id of the to-be-shared note
/// * `allow_req` - The body of the request parsed to a Vector containing ShareRequest-objects
/// * `user_manager` - A [`UserManager`]-instance of the requesting user
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
///         "success": true,
///         "time": "2022-04-11 12:05:57"
///     }
/// ```
/// ```text
/// PUT-Request at `{api-url}/share/72}4fa97$b62u3:2dr4d3l` with a cookie containing a JWT
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
/// => 400
///     {
///         "success": false,
///         "code": 21,
///         "message": "requested id contains forbidden character",
///         "time": "2022-04-11 12:20:19"
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
///         "success": false,
///         "code": 10,
///         "message": "user is not logged in",
///         "time": "2022-04-11 12:20:19"
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
///         "success": false,
///         "code": 12,
///         "message": "no permission",
///         "time": "2022-04-11 12:20:19"
///     }
/// ```
#[put("/share/{note_id}")]
pub async fn update_allowances(path: Path<String>, allow_req: web::Json<Vec<ShareRequest>>, user_manager: Box<dyn UserManager>) -> impl Responder {
    match user_manager.get_note(&path.into_inner()).await {
        Ok(note_manager) => {
            let mut failed_users = Vec::new();
            for share in allow_req.into_inner() {
                if note_manager.update_share(&share.user_id, share.allowance).await.is_err() {
                    failed_users.push(share.user_id);
                }
            }
            if !failed_users.is_empty() {
                return Err(APIError::QueryError(format!("failed to update allowances of the following user(s): [{}]",
                                                    failed_users.join(","))))
            }
            Ok(HttpResponse::Ok().json(ResponseObject::new()))
        },
        Err(DBError::InvalidSequenceError(_)) => Err(APIError::InvalidIDError),
        Err(DBError::MissingEntryError(_,_)) => Err(APIError::InvalidIDError),
        Err(_) => Err(APIError::QueryError("failed to access note".to_string()))
    }
}

/// Creates an invite-code with the inviting users name as its payload
///
/// # Arguments
///
/// * `uid` - The username to save
fn gen_invite(uid: &str) -> Result<String, APIError> {
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
    encode(&header, &claims, &EncodingKey::from_secret(env::var(SHARE_SECRET_ENV_VAR_KEY).unwrap().as_bytes()))
        .map_err(|_| APIError::InternalServerError("invite-code-creation failed".to_string()))
}

/// Retrieves the username of the inviting user from the invite-code
///
/// # Arguments
///
/// * `code` - Invite-code to be verified
fn get_user_id_from_invite_code(code: &String) -> Result<String, APIError> {
    decode::<Claims>(&code,
                     &DecodingKey::from_secret(env::var(SHARE_SECRET_ENV_VAR_KEY).unwrap().as_bytes()),
                     &Validation::new(Algorithm::HS256))
        .map(|dec|dec.claims.sub).map_err(|_| APIError::InvalidInviteError)
}
