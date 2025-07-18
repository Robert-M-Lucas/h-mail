mod handle_client;
mod main_loop_body;

use crate::database::conn::DatabaseRef;
use crate::inter_server_comm::recv::main_loop_body::main_loop_body;
use crate::log::LogSeverity::Info;
use crate::log::LogSource::Receiver;
use crate::log::{fatal_error, log};
use crate::manager::StopRunningFn;
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

pub fn start_receiver(db: DatabaseRef) -> StopRunningFn {
    log(Receiver, Info, "Starting receiver");
    let stop_flag = Arc::new(AtomicBool::new(false));
    let stop_flag_clone = stop_flag.clone();

    let t = thread::spawn(move || {
        let Ok(listener) = TcpListener::bind("0.0.0.0:8080") else {
            fatal_error(Receiver, "Failed to bind listener");
        };
        if listener.set_nonblocking(true).is_err() {
            fatal_error(Receiver, "Failed to set listener to non-blocking");
        }
        log(Receiver, Info, "Receiver started");
        while !stop_flag_clone.load(Ordering::Relaxed) {
            main_loop_body(&db, &listener);
        }
    });

    Box::new(move || {
        log(Receiver, Info, "Sending stop signal to receiver");
        stop_flag.store(true, Ordering::Relaxed);
        log(Receiver, Info, "Waiting for receiver thread to join");
        t.join().unwrap();
        log(Receiver, Info, "Receiver stopped");
    })
}
