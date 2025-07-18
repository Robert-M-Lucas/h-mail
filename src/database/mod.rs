pub mod conn;

use crate::database::conn::DatabaseRef;
use crate::log::LogSeverity::Info;
use crate::log::LogSource::Database;
use crate::log::log;
use crate::manager::StopRunningFn;

pub fn start_database() -> (DatabaseRef, StopRunningFn) {
    log(Database, Info, "Starting database");
    let db = DatabaseRef::connect();
    log(Database, Info, "Database started");

    (
        db.clone(),
        Box::new(move || {
            log(Database, Info, "Closing database");
            db.close();
            log(Database, Info, "Database closed");
        }),
    )
}
