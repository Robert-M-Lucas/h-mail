use rusqlite::{params, Connection};
use std::fs;
use crate::root::communication::interface::shared::{PowClassification, PowPolicy};

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
                "INSERT INTO Emails (user_id, email, pow_classification) VALUES (?1, ?2, ?3)",
                params![user_id, email, classification.to_ident()],
            )
            .unwrap();

        true
    }

    pub fn close(self) {
        println!("Closing database");
        self.connection.close().unwrap();
    }
}
