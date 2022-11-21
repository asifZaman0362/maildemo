use actix_session::Session;
use core::fmt;
use std::fmt::Formatter;

use pbkdf2::password_hash::PasswordHash;

use crate::database::Database;
use crate::encryption::hasher;

#[derive(Debug)]
pub enum AuthError {
    InvalidUsername,
    InvalidPassword,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Authentication error: {}",
            match self {
                AuthError::InvalidPassword => "InvalidPassword",
                AuthError::InvalidUsername => "InvalidUsername",
            }
        )
    }
}

pub fn check_auth(session: &Session) -> bool {
    // check if user is in session
    if let Ok(result) = session.get::<String>("username") {
        result.is_some()
    } else {
        false
    }
}

pub fn login(
    email_addr: &str,
    password: &str,
    database: &mut Database,
    session: &Session,
) -> Result<(), AuthError> {
    if let Ok(_) = database.get_user(email_addr) {
        let hash_str = database.get_password_hash(email_addr).unwrap();
        let hash = PasswordHash::new(&hash_str).unwrap();
        if hasher::verify_hash(password, &hash) {
            session.insert("username", String::from(email_addr)).unwrap();
            Ok(())
        } else {
            Err(AuthError::InvalidPassword)
        }
    } else {
        Err(AuthError::InvalidUsername)
    }
}

pub fn logout(session: &Session) {
    session.remove("username");
}
