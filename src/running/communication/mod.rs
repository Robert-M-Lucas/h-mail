use std::net::TcpListener;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use axum::Router;
use tokio::runtime::Runtime;
use crate::manager::StopRunningFn;
use crate::running::database::conn::DatabaseRef;
use crate::running::log::log;
use crate::running::log::log::{fatal_error, log};
use crate::running::log::log_types::LogSeverity::{Error, Info};
use crate::running::log::log_types::LogSource::{Comms, Receiver};

pub mod recv;
pub mod send;

pub fn start_communication(db: DatabaseRef) -> StopRunningFn {
    log::log(Comms, Info, "Starting communications");

    let stop_flag = Arc::new(AtomicBool::new(false));
    let stop_flag_clone = stop_flag.clone();

    let t = thread::spawn(move || {
        let Ok(rt) = Runtime::new() else {
            fatal_error(Comms, "Failed to start tokio runtime");
        };

        let app = Router::new();
        let Ok(listener) = rt.block_on(tokio::net::TcpListener::bind("0.0.0.0:8080")) else {
            fatal_error(Comms, "Failed to bind TcpListener")
        };

        let f = axum::serve(listener, app).with_graceful_shutdown(
            async move {
                while !stop_flag_clone.load(Ordering::Relaxed) {
                    tokio::time::sleep(Duration::from_millis(100)).await
                }
            }
        );

        if let Err(e) = rt.block_on(f.into_future()) {
            fatal_error(Comms, format!("Tokio runtime exited with error: {e}"))
        };

    });

    Box::new(move || {
        log::log(Comms, Info, "Sending stop signal to communications");
        stop_flag.store(true, Ordering::Relaxed);
        log::log(Comms, Info, "Waiting for communications thread to join");
        t.join().unwrap();
        log::log(Comms, Info, "Communications stopped");
    })
}
