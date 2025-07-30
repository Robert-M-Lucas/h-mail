use crate::config::DEFAULT_USER_POW_POLICY;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use h_mail_interface::interface::email::EmailContents;
use h_mail_interface::interface::pow::{PowClassification, PowPolicy};
use h_mail_interface::interface::routes::native::get_emails::GetEmailsEmail;
use itertools::Itertools;
use rusqlite::fallible_iterator::FallibleIterator;
use std::{env, fs};
use diesel::Connection;
use diesel::prelude::*;
use crate::database;
use crate::database::schema::Users::dsl::Users;

mod schema;

pub type UserId = i64;

pub struct Database {
    connection: SqliteConnection,
}

impl Database {
    pub fn connect() -> Database {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let connection = SqliteConnection::establish(&database_url)
            .expect("Error connecting to the database");

        Database { connection }
    }

    fn get_salt() -> SaltString {
        #[cfg(feature = "no_salt")]
        let salt = [0u8; 8];
        #[cfg(not(feature = "no_salt"))]
        compile_error!("no_salt feature must be enabled. Salt functionality not implemented");

        SaltString::encode_b64(&salt).unwrap()
    }

    pub fn create_user(&self, username: &str, password: &str) -> Result<(), ()> {
        // let mut stmt = self.connection.prepare("INSERT INTO Users (username, password_hash, pow_minimum, pow_accepted, pow_personal) VALUES (?1, ?2, ?3, ?4, ?5)").unwrap();
        let salt_string = Self::get_salt();

        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt_string)
            .unwrap();

        diesel::insert_into(Users).values(()).

        // if stmt.execute(params![
        //     username,
        //     password_hash.to_string(),
        //     DEFAULT_USER_POW_POLICY.minimum(),
        //     DEFAULT_USER_POW_POLICY.accepted(),
        //     DEFAULT_USER_POW_POLICY.personal(),
        // ]).is_err() {
        //     return Err(());
        // }
        // Ok(())
    }

    pub fn authenticate(&self, username: &str, password: &str) -> Result<UserId, ()> {
        let mut stmt = self
            .connection
            .prepare("SELECT user_id from Users WHERE username = ?1 AND password_hash = ?2")
            .unwrap();

        let salt_string = Self::get_salt();

        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt_string)
            .unwrap();

        let Ok(user_id): rusqlite::Result<UserId> = stmt
            .query_row(params![username, password_hash.to_string()], |row| {
                row.get(0)
            })
        else {
            return Err(());
        };

        Ok(user_id)
    }

    pub fn has_user(&self, user: &str) -> bool {
        let mut stmt = self
            .connection
            .prepare("SELECT 1 FROM Users WHERE username = ?1 LIMIT 1")
            .unwrap();
        let mut rows = stmt.query([user]).unwrap();
        rows.next().unwrap().is_some()
    }

    pub fn get_username_from_id(&self, id: UserId) -> Option<String> {
        let mut stmt = self
            .connection
            .prepare("SELECT username FROM Users WHERE user_id = ?1")
            .unwrap();

        let Ok(username): rusqlite::Result<String> = stmt.query_row(params![id], |row| row.get(0))
        else {
            return None;
        };

        Some(username)
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
        email: &EmailContents,
        classification: PowClassification,
    ) -> Result<(), ()> {
        let Ok(user_id): rusqlite::Result<UserId> = self.connection.query_row(
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
                params![user_id, source, email.contents(), classification.to_ident()],
            )
            .unwrap();

        Ok(())
    }

    pub fn get_emails(&self, authed_user: UserId, since: i64) -> Vec<GetEmailsEmail> {
        // let Ok(user_id): rusqlite::Result<i64> = self.connection.query_row(
        //     "SELECT user_id FROM Users WHERE username = ?1",
        //     [authed_user],
        //     |row| row.get(0),
        // ) else {
        //     return None;
        // };

        let mut stmt = self.connection.prepare(
            "SELECT source, email, pow_classification FROM Emails WHERE user_id = ?1 AND email_id >= ?2",
        ).unwrap();

        let rows = stmt.query(params![authed_user, since]).unwrap();

        rows.map(|row| {
            let pow_classification: String = row.get(2).unwrap();
            Ok(GetEmailsEmail::new(
                row.get(0).unwrap(),
                row.get(1).unwrap(),
                PowClassification::from_ident(&pow_classification).unwrap(),
            ))
        })
        .unwrap()
        .collect_vec()
    }

    pub fn close(self) {
        println!("Closing database");
        self.connection.close().unwrap();
    }
}
