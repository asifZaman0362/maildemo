use pbkdf2::password_hash::PasswordHash;
use std::path::Path;
use rusqlite::{self, Connection, Result};
use crate::types::{Email, RegisterData};
use crate::encryption::{cipher, hasher};

#[derive(Debug)]
pub struct Database {
    pub connection: Connection
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
        let mut statement = self.connection.prepare("SELECT * FROM USER WHERE email = ?")?;
        let mut rows = statement.query([username])?;
        if let Some(user) = rows.next()? {
            Ok(user.get(0)?)
        } else { Err(rusqlite::Error::QueryReturnedNoRows) }
    }
    pub fn has_user(&mut self) -> Result<()> {
        let mut statement = self.connection.prepare("SELECT * FROM USER")?;
        let mut rows = statement.query([])?;
        if let Some(_) = rows.next()? {
            return Ok(());
        }
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }
    pub fn get_password_hash(&mut self, username: &str) -> Result<String> {
        let mut statement = self.connection.prepare("SELECT * FROM USER WHERE email = ?")?;
        let mut rows = statement.query([username])?;
        if let Some(user) = rows.next()? {
            let hash_str: String = user.get(1)?;
            Ok(hash_str)
        } else {
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }
    pub fn add_user(&mut self, email: &str, password: &str, firstname: &str, lastname: &str) -> Result<()> {
        let mut statement = self.connection.prepare("INSERT INTO USER VALUES(?, ?, ?, ?)")?;
        let hash = hasher::compute_hash(&password).unwrap();
        statement.execute([email.to_owned(), hash, firstname.to_owned(), lastname.to_owned()])?;
        Ok(())
    }
}
