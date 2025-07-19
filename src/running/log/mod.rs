use std::sync::OnceLock;
use crossbeam::channel::Sender;
use crate::running::log::log_types::LogPacket;

pub mod log_types;
pub mod logger;
pub mod log;

static TUI_QUEUE: OnceLock<Sender<LogPacket>> = OnceLock::new();
static LOGGER_QUEUE: OnceLock<Sender<LogPacket>> = OnceLock::new();