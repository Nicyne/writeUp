//! Storage-solution for various database-systems

mod interface;
mod error;

/// Chars not serving a use outside of a potential injection-attempt
pub const FORBIDDEN_CHARS:[char;4] = ['{', '}', '$', ':']; //TODO? Check for '.' (only used in jwt so far)

/// All supported database-driver
pub enum Driver {
    MongoDB
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