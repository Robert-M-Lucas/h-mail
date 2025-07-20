use derive_getters::Getters;
use rusqlite::Connection;
use std::fs;
use std::sync::{Arc, Mutex};
use serde::Serialize;

#[derive(Getters, Serialize)]
pub struct PowPolicy {
    minimum: u32,
    accepted: u32,
    personal: u32,
}


pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn connect() -> Database {
        fs::create_dir("data").ok();
        let connection = Connection::open("data/data.db").unwrap();
        connection.execute("CREATE TABLE IF NOT EXISTS Users (
    username TEXT PRIMARY KEY,
    pow_minimum INTEGER,
    pow_accepted INTEGER,
    pow_personal INTEGER)", ()).unwrap();
        
        Database {
            connection,
        }
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
}
