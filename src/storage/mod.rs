//! Storage-solution for various database-systems

use std::env;
use argonautica::{Hasher, Verifier};
use crate::PASSWD_SECRET_ENV_VAR_KEY;
use crate::storage::error::DBError;
use crate::storage::interface::ManagerPool;
use crate::storage::mongodb::MongoDBPool;

pub mod interface;
pub mod error;
mod mongodb;

/// Chars not serving a use outside of a potential injection-attempt
const FORBIDDEN_CHARS:[char;6] = ['{', '}', '$', ':', '-', '%']; //TODO? Check for '.' (only used in jwt so far)

/// All supported database-driver
pub enum Driver {
    MongoDB
}

impl Driver {
    pub async fn get_pool(&self, uri: (String, String), cred: (String, String)) -> Result<Box<dyn ManagerPool>, DBError> {
        match self {
            Driver::MongoDB => MongoDBPool::new(uri, cred).await
        }
    }
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
/// use crate::storage::is_safe;
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

///Generates a password hash to be stored in the db
///
/// # Arguments
///
/// * `passwd` - A string slice containing the password to be hashed
pub fn gen_hash(password: &str) -> String {
    /// Available space in memory per each hash
    const HASH_MEM_SIZE: u32 = 65536; //kiB
    /// Amount of iterations to be done per hash
    const HASH_ITER_COUNT: u32 = 8;

    let pepper = env::var(PASSWD_SECRET_ENV_VAR_KEY).unwrap();

    let mut hasher = Hasher::default();
    hasher.configure_memory_size(HASH_MEM_SIZE)
        .configure_iterations(HASH_ITER_COUNT)
        .with_secret_key(pepper);

    hasher.with_password(password).hash().unwrap()
}

/// Compares a given password with the one associated with the account
///
/// # Arguments
///
/// * `passwd` - A string slice containing the supposed password in plain text
pub fn verify(password: &str, password_hash: &str) -> bool {
    let pepper = env::var(PASSWD_SECRET_ENV_VAR_KEY).unwrap();

    let mut verifier = Verifier::default();
    verifier.with_secret_key(pepper);
    verifier.with_hash(password_hash).with_password(password);

    match verifier.verify() {
        Ok(ver) => ver,
        Err(_) => false // cant process hash
    }
}