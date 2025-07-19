use std::sync::OnceLock;
use crossbeam::channel::Sender;
use chrono::Utc;
use crate::manager::stop_all::stop_all;
use crate::running::log::log_types::LogSeverity::Fatal;
use crate::running::log::log_types::{LogPacket, LogSeverity, LogSource};
use crate::running::log::{LOGGER_QUEUE, TUI_QUEUE};

pub fn fatal_error<T: AsRef<str>>(source: LogSource, message: T) -> ! {
    log(
        source,
        Fatal,
        format!("Fatal: {} - Stopping all.", message.as_ref()),
    );
    stop_all(message);
}

pub fn log<T: AsRef<str>>(source: LogSource, severity: LogSeverity, message: T) {
    let packet = LogPacket {
        timestamp: Utc::now(),
        source: source.clone(),
        severity: severity.clone(),
        message: message.as_ref().to_string(),
    };

    if TUI_QUEUE
        .get()
        .expect("Log used before logger initialisation")
        .send(packet.clone())
        .is_err()
    {
        let severity = severity.get_name();
        let source = source.get_name();
        println!(
            "[{}] [{severity}] [{source}] {}",
            Utc::now().format("%H:%M:%S"),
            message.as_ref()
        );
    }

    LOGGER_QUEUE
        .get()
        .expect("Log used before logger initialisation")
        .send(packet)
        .ok();
}