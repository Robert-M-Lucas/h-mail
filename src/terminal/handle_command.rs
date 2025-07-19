use crate::log::LogSeverity::{Error, Info, Warn};
use crate::log::LogSource::Tui;
use crate::log::log;
use crate::manager::stop_all::stop_all;
use crate::terminal::cli::Cli;
use clap::Parser;
use std::{iter, thread};

pub fn handle_command(input: &str) {
    log(Tui, Info, format!("Executing command: `{input}`"));
    let cli = match Cli::try_parse_from(iter::once(" ").chain(input.split_whitespace())) {
        Ok(cli) => cli,
        Err(e) => {
            log(Tui, Error, format!("{e}"));
            return;
        }
    };
    match cli {
        Cli::Send => {
            log(Tui, Warn, "Send is unimplemented");
        }
        Cli::Exit => {
            thread::spawn(|| stop_all());
        }
    }
}
