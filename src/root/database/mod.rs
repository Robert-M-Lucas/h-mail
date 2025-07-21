use derive_getters::Getters;
use derive_new::new;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Getters, Serialize, Deserialize, new, Debug)]
pub struct PowPolicy {
    minimum: u64,
    accepted: u64,
    personal: u64,
}

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn connect() -> Database {
        fs::create_dir("data").ok();
        let connection = Connection::open("data/data.db").unwrap();
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS Users (
    username TEXT PRIMARY KEY,
    pow_minimum INTEGER,
    pow_accepted INTEGER,
    pow_personal INTEGER)",
                (),
            )
            .unwrap();

        Database { connection }
    }

    pub fn has_user(&self, user: &str) -> bool {
        true
    }

    pub fn get_user_pow_policy(&self, user: &str) -> Option<PowPolicy> {
        Some(PowPolicy {
            minimum: 1000,
            accepted: 6_500,
            personal: 65_000,
        })
    }
    
    pub fn deliver_email(&mut self, user: &str, email: &str, iters: u64, policy: PowPolicy) -> bool {
        true
    }
}
