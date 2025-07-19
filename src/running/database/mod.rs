pub mod conn;

use crate::running::database::conn::DatabaseRef;
use crate::running::log::log_types::LogSeverity::Info;
use crate::running::log::log_types::LogSource::Database;
use crate::running::log::log;
use crate::manager::StopRunningFn;

pub fn start_database() -> (DatabaseRef, StopRunningFn) {
    log::log(Database, Info, "Starting database");
    let db = DatabaseRef::connect();
    log::log(Database, Info, "Database started");

    (
        db.clone(),
        Box::new(move || {
            log::log(Database, Info, "Closing database");
            db.close();
            log::log(Database, Info, "Database closed");
        }),
    )
}
