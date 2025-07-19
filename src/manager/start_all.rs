use std::io::{stdout, Write};
use crate::running::database::start_database;
use crate::running::communication::recv::start_receiver;
use crate::running::log::logger::start_logger;
use crate::manager::running::{FUNCTION_LOCK, LOG_FILE, RUNNING};
use crate::running::terminal::main_loop::{get_terminal_flag, start_tui_blocking};
use itertools::Itertools;
use std::sync::Mutex;
use crate::running::communication::start_communication;

pub fn start_all() {
    let mut running = Vec::new();

    let ((tui_rx, log_file), stop_log) = start_logger();
    if LOG_FILE.set(log_file).is_err() {
        panic!("start_all called twice")
    }
    
    let (tui_flag, stop_tui) = get_terminal_flag();


    let (db, stop_db) = start_database();
    running.push(("Database", stop_db));

    stdout().flush().ok();

    let stop_comms = start_communication(db.clone());
    running.push(("Communications", stop_comms));

    running.push(("TUI", stop_tui));
    running.push(("Logger", stop_log));

    if RUNNING
        .set(
            running
                .into_iter()
                .map(|(n, f)| (n, Mutex::new(Some(f))))
                .collect_vec(),
        )
        .is_err()
    {
        panic!("start_all called twice")
    }

    start_tui_blocking(tui_rx, tui_flag);
}
