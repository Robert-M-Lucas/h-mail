use crate::running::log::log_types::LogSeverity::{Error, Info, Warn};
use crate::running::log::log_types::LogSource::Tui;
use crate::running::log::log;
use crate::manager::stop_all::stop_all;
use crate::running::terminal::cli::Cli;
use clap::Parser;
use std::{iter, thread};

pub fn handle_command(input: &str) {
    log::log(Tui, Info, format!("Executing command: `{input}`"));
    let cli = match Cli::try_parse_from(iter::once(" ").chain(input.split_whitespace())) {
        Ok(cli) => cli,
        Err(e) => {
            log::log(Tui, Error, format!("{e}"));
            return;
        }
    };
    match cli {
        Cli::Send => {
            log::log(Tui, Warn, "Send is unimplemented");
        }
        Cli::Exit => {
            thread::spawn(|| stop_all("TUI 'exit' command"));
        }
    }
}
