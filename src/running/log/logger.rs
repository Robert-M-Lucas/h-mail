use crossbeam::channel::{unbounded, Receiver, Sender, TryRecvError};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{fs, thread};
use std::fs::{File, OpenOptions};
use chrono::Utc;
use std::time::Duration;
use itertools::Itertools;
use std::io::Write;
use std::path::PathBuf;
use crate::manager::StopRunningFn;
use crate::running::log;
use crate::running::log::log_types::LogPacket;
use crate::running::log::log_types::LogSeverity::Info;
use crate::running::log::log_types::LogSource::Logger;
use crate::running::log::{LOGGER_QUEUE, TUI_QUEUE};

pub fn start_logger() -> ((Receiver<LogPacket>, PathBuf), StopRunningFn) {
    let (tui_tx, tui_rx): (Sender<LogPacket>, Receiver<LogPacket>) = unbounded();
    let (log_tx, log_rx): (Sender<LogPacket>, Receiver<LogPacket>) = unbounded();

    TUI_QUEUE.set(tui_tx).unwrap();
    LOGGER_QUEUE.set(log_tx).unwrap();

    log::log::log(Logger, Info, "Starting logger");

    let stop_flag = Arc::new(AtomicBool::new(false));
    let stop_flag_clone = stop_flag.clone();

    fs::create_dir("../../../logs").ok();
    let path = PathBuf::from(Utc::now().format("logs/%Y-%m-%d_%H-%M-%S.log").to_string());
    let Ok(mut file) = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&path)
    else {
        log::log::fatal_error(Logger, "Failed to create log file");
    };
    
    let t = thread::spawn(move || {
        log::log::log(Logger, Info, "Logger started");
        while !stop_flag_clone.load(Ordering::Relaxed) {
            poll_log_to_file(&mut file, &log_rx);
        }
        poll_log_to_file(&mut file, &log_rx);
    });

    (
        (tui_rx, path),
        Box::new(move || {
            log::log::log(Logger, Info, "Sending stop signal to logger");
            stop_flag.store(true, Ordering::Relaxed);
            log::log::log(Logger, Info, "Waiting for logger thread to join");
            t.join().unwrap();
            log::log::log(Logger, Info, "Logger stopped");
        }),
    )
}

fn poll_log_to_file(file: &mut File, log_rx: &Receiver<LogPacket>) {
    let mut current = String::new();
    loop {
        match log_rx.try_recv() {
            Ok(msg) => {
                let timestamp = msg.timestamp();
                let severity = msg.severity().get_name();
                let source = msg.source().get_name();
                let message = msg.message();
                current += &format!(
                    "[{}] [{severity}] [{source}] {}",
                    timestamp.format("%H:%M:%S"),
                    message.lines().join("\n> ")
                );
                current.push('\n');
            }
            Err(TryRecvError::Empty) => {
                break;
            }
            Err(TryRecvError::Disconnected) => {
                log::log::fatal_error(Logger, "Logger channel disconnected");
            }
        }
    }
    if current.is_empty() {
        thread::sleep(Duration::from_millis(100));
        return;
    }
    if file.write_all(current.as_bytes()).is_err() || file.sync_all().is_err() {
        log::log::fatal_error(Logger, "Failed to write to file");
    }
}