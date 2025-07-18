use crate::log::LogSeverity::{Error, Fatal, Info};
use crate::log::LogSource::Logger;
use crate::manager::StopRunningFn;
use crate::manager::stop_all::stop_all;
use chrono::{DateTime, Utc};
use color_print::cformat;
use crossbeam::channel::{Receiver, Sender, TryRecvError, unbounded};
use derive_getters::Getters;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::net::TcpListener;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use std::{fs, thread};
use itertools::Itertools;
use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone)]
pub enum LogSource {
    Manager,
    Database,
    Receiver,
    Logger,
    TUI,
}

impl LogSource {
    pub fn get_name(&self) -> &'static str {
        match self {
            LogSource::Manager => "Manager",
            LogSource::Database => "Database",
            LogSource::Receiver => "Receiver",
            LogSource::Logger => "Logger",
            LogSource::TUI => "TUI",
        }
    }
}

#[derive(Debug, Clone)]
pub enum LogSeverity {
    Info,
    Warn,
    Error,
    Fatal,
}

impl LogSeverity {
    pub fn get_name(&self) -> &'static str {
        match self {
            LogSeverity::Info => "Info",
            LogSeverity::Warn => "Warn",
            LogSeverity::Error => "Error",
            LogSeverity::Fatal => "Fatal",
        }
    }

    pub fn get_style(&self) -> Style {
        match self {
            LogSeverity::Info => Style::default().fg(Color::Cyan),
            LogSeverity::Warn => Style::default().fg(Color::Yellow),
            LogSeverity::Error => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            LogSeverity::Fatal => Style::default().fg(Color::White).bg(Color::Red).add_modifier(Modifier::BOLD),
        }
    }
}

static TUI_QUEUE: OnceLock<Sender<LogPacket>> = OnceLock::new();
static LOGGER_QUEUE: OnceLock<Sender<LogPacket>> = OnceLock::new();

pub fn fatal_error<T: AsRef<str>>(source: LogSource, message: T) -> ! {
    log(
        source,
        Fatal,
        format!("Fatal: {} - Stopping all.", message.as_ref()),
    );
    stop_all();
}

#[derive(Getters, Debug, Clone)]
pub struct LogPacket {
    pub timestamp: DateTime<Utc>,
    pub source: LogSource,
    pub severity: LogSeverity,
    pub message: String,
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

pub fn start_logger() -> (Receiver<LogPacket>, StopRunningFn) {
    let (tui_tx, tui_rx): (Sender<LogPacket>, Receiver<LogPacket>) = unbounded();
    let (log_tx, log_rx): (Sender<LogPacket>, Receiver<LogPacket>) = unbounded();

    TUI_QUEUE.set(tui_tx).unwrap();
    LOGGER_QUEUE.set(log_tx).unwrap();

    log(Logger, Info, "Starting logger");

    let stop_flag = Arc::new(AtomicBool::new(false));
    let stop_flag_clone = stop_flag.clone();

    let t = thread::spawn(move || {
        fs::create_dir("logs").ok();
        let Ok(mut file) = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(Utc::now().format("logs/%Y-%m-%d_%H-%M-%S.log").to_string())
        else {
            fatal_error(Logger, "Failed to create log file");
        };
        log(Logger, Info, "Logger started");
        while !stop_flag_clone.load(Ordering::Relaxed) {
            poll_log_to_file(&mut file, &log_rx);
        }
        poll_log_to_file(&mut file, &log_rx);
        ()
    });

    (
        tui_rx,
        Box::new(move || {
            log(Logger, Info, "Sending stop signal to logger");
            stop_flag.store(true, Ordering::Relaxed);
            log(Logger, Info, "Waiting for logger thread to join");
            t.join().unwrap();
            log(Logger, Info, "Logger stopped");
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
                fatal_error(Logger, "Logger channel disconnected");
            }
        }
    }
    if current.is_empty() {
        thread::sleep(Duration::from_millis(100));
        return;
    }
    if file.write_all(current.as_bytes()).is_err() || file.sync_all().is_err() {
        fatal_error(Logger, "Failed to write to file");
    }
}
