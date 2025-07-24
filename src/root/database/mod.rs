use crate::root::receiving::interface::get_emails::{GetEmailsEmail, GetEmailsResponse};
use crate::root::receiving::interface::shared::{PowClassification, PowPolicy};
use itertools::Itertools;
use rusqlite::fallible_iterator::FallibleIterator;
use rusqlite::{Connection, Rows, params};
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
        source: &str,
        email: &str,
        classification: PowClassification,
    ) -> bool {
        let Ok(user_id): rusqlite::Result<i64> = self.connection.query_row(
            "SELECT user_id FROM Users WHERE username = ?1",
            [user],
            |row| row.get(0),
        ) else {
            return false;
        };

        self.connection
            .execute(
                "INSERT INTO Emails (user_id, source, email, pow_classification) VALUES (?1, ?2, ?3, ?4)",
                params![user_id, source, email, classification.to_ident()],
            )
            .unwrap();

        true
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

        let mut rows = stmt.query(params![user_id, since]).unwrap();

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
