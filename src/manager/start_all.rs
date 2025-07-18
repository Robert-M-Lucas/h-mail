use crate::database::start_database;
use crate::inter_server_comm::recv::start_receiver;
use crate::log::start_logger;
use crate::manager::running::{FUNCTION_LOCK, RUNNING};
use crate::terminal::main_loop::{get_terminal_flag, start_tui_blocking};
use itertools::Itertools;
use std::sync::Mutex;

pub fn start_all() {
    let guard = FUNCTION_LOCK.lock().unwrap();
    let mut running = Vec::new();

    let (tui_rx, stop_log) = start_logger();

    let (tui_flag, stop_tui) = get_terminal_flag();

    let (db, stop_db) = start_database();
    running.push(("Database", stop_db));

    let stop_recv = start_receiver(db.clone());
    running.push(("Receiver", stop_recv));

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

    drop(guard);
    start_tui_blocking(tui_rx, tui_flag);
}
