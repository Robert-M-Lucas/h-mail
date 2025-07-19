use ratatui::prelude::{Color, Modifier, Style};
use derive_getters::Getters;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub enum LogSource {
    Manager,
    Database,
    Comms,
    Receiver,
    Sender,
    Logger,
    Tui,
}

impl LogSource {
    pub fn get_name(&self) -> &'static str {
        match self {
            LogSource::Manager => "Manager",
            LogSource::Database => "Database",
            LogSource::Comms => "Comms",
            LogSource::Receiver => "Comms/Recv",
            LogSource::Sender => "Comms/Send",
            LogSource::Logger => "Logger",
            LogSource::Tui => "TUI",
            
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
            LogSeverity::Fatal => Style::default()
                .fg(Color::White)
                .bg(Color::Red)
                .add_modifier(Modifier::BOLD),
        }
    }
}

#[derive(Getters, Debug, Clone)]
pub struct LogPacket {
    pub timestamp: DateTime<Utc>,
    pub source: LogSource,
    pub severity: LogSeverity,
    pub message: String,
}