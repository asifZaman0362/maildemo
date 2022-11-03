use rusqlite::{ToSql, types::ToSqlOutput};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EmailAddress {
    pub username: String,
    pub host: String
}

impl ToString for EmailAddress {
    fn to_string(&self) -> String {
        format!("{}@{}", self.username, self.host)
    }
}

impl ToSql for EmailAddress {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        let string = self.to_string();
        Ok(ToSqlOutput::from(string))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Email {
    pub mail_id: String,
    pub subject: String,
    pub sender_address: Option<EmailAddress>,
    pub receiver_address: EmailAddress,
    pub content: String,
    pub date: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attachment {
    pub attachment_id: String,
    pub filename: String,
    pub mail_id: String
}

#[derive(Debug)]
pub struct Session {
    pub username: Option<String>
}
