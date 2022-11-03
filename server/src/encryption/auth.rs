use pbkdf2::password_hash::PasswordHash;

use crate::database::Database;
use crate::encryption::hasher;
use crate::types::Session;

pub enum AuthError {
    InvalidUsername,
    InvalidPassword
}

pub fn check_auth(session: &Session) -> bool {
    // check if user is in session
    if let Some(_) = session.username { true }
    else { false }
}

pub fn login(email_addr: &str, password: &str, database: &mut Database, session: &mut Session) -> Result<(), AuthError> {
    if let Ok(_) = database.get_user(email_addr) {
        let hash_str = database.get_password_hash(email_addr).unwrap();
        let hash = PasswordHash::new(&hash_str).unwrap();
        if hasher::verify_hash(password, &hash) {
            session.username = Some(String::from(email_addr));
            Ok(())
        } else {
            Err(AuthError::InvalidPassword)
        }
    } else {
        Err(AuthError::InvalidUsername)
    }
}

pub fn logout(session: &mut Session) {
    session.username = None;
}
