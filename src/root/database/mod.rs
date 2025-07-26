use crate::root::config::DEFAULT_USER_POW_POLICY;
use crate::root::receiving::interface::get_emails::GetEmailsEmail;
use crate::root::receiving::interface::shared::{PowClassification, PowPolicy};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use itertools::Itertools;
use rusqlite::fallible_iterator::FallibleIterator;
use rusqlite::{params, Connection};
use std::fs;

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn connect() -> Database {
        fs::create_dir("data").ok();
        let connection = Connection::open("data/data.db").unwrap();

        connection.execute("PRAGMA foreign_keys = ON;", []).unwrap();

        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS Users (
    user_id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    pow_minimum INTEGER NOT NULL,
    pow_accepted INTEGER NOT NULL,
    pow_personal INTEGER NOT NULL)",
                (),
            )
            .unwrap();

        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS Emails (
    email_id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    source TEXT NOT NULL,
    email TEXT NOT NULL,
    pow_classification TEXT CHECK(pow_classification IN ('MINIMUM', 'ACCEPTED', 'PERSONAL')),
    FOREIGN KEY (user_id) REFERENCES Users(user_id) ON DELETE CASCADE)",
                (),
            )
            .unwrap();

        Database { connection }
    }

    pub fn create_user(&self, username: &str, password: &str) -> Result<(), ()> {
        let mut stmt = self.connection.prepare("INSERT INTO Users (username, password_hash, pow_minimum, pow_accepted, pow_personal) VALUES (?1, ?2, ?3, ?4, ?5)").unwrap();
        
        #[cfg(feature = "no_salt")]
        let salt = [0u8; 8];
        #[cfg(not(feature = "no_salt"))]
        compile_error!("no_salt feature must be enabled. Salt functionality not implemented");
        
        let salt_string = SaltString::encode_b64(&salt).unwrap();
        let password_hash = Argon2::default().hash_password(password.as_bytes(), &salt_string).unwrap();

        if let Err(e) = stmt.execute(params![
            username,
            password_hash.to_string(),
            DEFAULT_USER_POW_POLICY.minimum(),
            DEFAULT_USER_POW_POLICY.accepted(),
            DEFAULT_USER_POW_POLICY.personal(),
        ]) {
            return match e {
                _ => Err(())
            }
        }
        Ok(())
    }
    
    pub fn has_user(&self, user: &str) -> bool {
        let mut stmt = self
            .connection
            .prepare("SELECT 1 FROM Users WHERE username = ?1 LIMIT 1")
            .unwrap();
        let mut rows = stmt.query([user]).unwrap();
        rows.next().unwrap().is_some()
    }

    pub fn get_user_pow_policy(&self, user: &str) -> Option<PowPolicy> {
        let mut stmt = self.connection.prepare(
            "SELECT pow_minimum, pow_accepted, pow_personal FROM Users WHERE username = ?1 LIMIT 1",
        ).unwrap();

        let mut rows = stmt.query([user]).unwrap();

        if let Some(row) = rows.next().unwrap() {
            let pow_policy = PowPolicy::new(
                row.get(0).unwrap(),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
            );
            Some(pow_policy)
        } else {
            None
        }
    }

    pub fn deliver_email(
        &self,
        user: &str,
        source_user: &str,
        source_domain: &str,
        email: &str,
        classification: PowClassification,
    ) -> Result<(), ()> {
        let Ok(user_id): rusqlite::Result<i64> = self.connection.query_row(
            "SELECT user_id FROM Users WHERE username = ?1",
            [user],
            |row| row.get(0),
        ) else {
            return Err(());
        };

        let source = format!("{source_user}@{source_domain}");

        self.connection
            .execute(
                "INSERT INTO Emails (user_id, source, email, pow_classification) VALUES (?1, ?2, ?3, ?4)",
                params![user_id, source, email, classification.to_ident()],
            )
            .unwrap();

        Ok(())
    }

    pub fn get_emails(&self, user: &str, since: i64) -> Option<Vec<GetEmailsEmail>> {
        let Ok(user_id): rusqlite::Result<i64> = self.connection.query_row(
            "SELECT user_id FROM Users WHERE username = ?1",
            [user],
            |row| row.get(0),
        ) else {
            return None;
        };

        let mut stmt = self.connection.prepare(
            "SELECT source, email, pow_classification FROM Emails WHERE user_id = ?1 AND email_id >= ?2",
        ).unwrap();

        let rows = stmt.query(params![user_id, since]).unwrap();

        Some(
            rows.map(|row| {
                let pow_classification: String = row.get(2).unwrap();
                Ok(GetEmailsEmail::new(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    PowClassification::from_ident(&pow_classification).unwrap(),
                ))
            })
            .unwrap()
            .collect_vec(),
        )
    }

    pub fn close(self) {
        println!("Closing database");
        self.connection.close().unwrap();
    }
}
