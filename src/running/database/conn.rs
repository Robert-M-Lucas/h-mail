use crate::running::log::log_types::LogSource::Database;
use crate::running::log::log::fatal_error;
use derive_getters::Getters;
use rusqlite::Connection;
use std::fs;
use std::sync::{Arc, Mutex};

#[derive(Getters)]
pub struct PowPolicy {
    not_spam_threshold: u32,
    personal_threshold: u32,
}

#[derive(Clone)]
pub struct DatabaseRef {
    connection: Arc<Mutex<Option<Connection>>>,
}

impl DatabaseRef {
    pub fn connect() -> DatabaseRef {
        fs::create_dir("../../../data").ok();

        let Ok(connection) = Connection::open("../../../data/data.db") else {
            fatal_error(Database, "Failed to connect to database");
        };

        DatabaseRef {
            connection: Arc::new(Mutex::new(Some(connection))),
        }
    }

    pub fn close(&self) {
        self.connection
            .lock()
            .unwrap()
            .take()
            .unwrap()
            .close()
            .unwrap();
    }

    pub fn has_user(&self, user: &str) -> bool {
        true
    }

    pub fn get_user_pow_policy(&self, user: &str) -> PowPolicy {
        PowPolicy {
            not_spam_threshold: 10_000,
            personal_threshold: 100_000,
        }
    }
}
