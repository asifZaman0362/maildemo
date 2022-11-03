use pbkdf2::password_hash::PasswordHash;
use std::path::Path;
use rusqlite::{self, Connection, Result};
use crate::types::Email;
use crate::encryption::cipher;

pub struct Database {
    connection: Connection
}

impl Database {
    pub fn decrypt_database<T> (key: &[u8], database_file: T) -> bool 
    where T: AsRef<Path>
    {

        true
    }
    pub fn save_mail(&self, mail: Email) -> Result<usize> {
        self.connection.execute(
            "INSERT INTO EMAIL (mail_id, sender, receiver, subject, content, date) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (mail.mail_id, mail.sender_address, mail.receiver_address, mail.subject, mail.content, mail.date)
        )
    }
    pub fn get_user(&mut self, username: &str) -> Result<String> {
        let mut statement = self.connection.prepare("SELECT * FROM USER WHERE username = ?")?;
        let mut rows = statement.query([username])?;
        if let Some(user) = rows.next()? {
            Ok(user.get(0)?)
        } else { Err(rusqlite::Error::QueryReturnedNoRows) }
    }
    pub fn get_password_hash(&mut self, username: &str) -> Result<String> {
        let mut statement = self.connection.prepare("SELECT * FROM USER WHERE username = ?")?;
        let mut rows = statement.query([username])?;
        if let Some(user) = rows.next()? {
            let hash_str: String = user.get(2)?;
            Ok(hash_str)
        } else {
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }
}
