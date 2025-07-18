use crate::log::LogSeverity::Info;
use crate::log::LogSource::TUI;
use crate::log::{log, LogPacket};
use crate::manager::{StopRunningFn, stop_all};
use crate::terminal::cli::Cli;
use crate::terminal::handle_command::handle_command;
use clap::Parser;
use color_print::cprint;
use crossbeam::channel::Receiver;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::crossterm::{event, execute};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState};
use std::io::{Stdout, Write};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;
use std::{io, iter, thread};
use std::fmt::format;
use itertools::Itertools;
use ratatui::style::{Color, Style};

pub fn get_terminal_flag() -> (Arc<AtomicBool>, StopRunningFn) {
    let flag = Arc::new(AtomicBool::new(false));

    (
        flag.clone(),
        Box::new(move || {
            log(TUI, Info, "Setting flag to stop terminal");
            flag.store(true, std::sync::atomic::Ordering::Relaxed);
            log(TUI, Info, "Waiting for 1s terminal to exit");
            thread::sleep(Duration::from_secs(1));
        }),
    )
}

pub fn start_tui_blocking(tui_rx: Receiver<LogPacket>, stop_flag: Arc<AtomicBool>) {
    ctrlc::set_handler(move || {
        stop_all::stop_all();
    })
    .expect("Error setting Ctrl-C handler");

    tui_wrapper(tui_rx, stop_flag).unwrap();

    log(TUI, Info, "TUI stopped");
}

fn tui_wrapper(
    tui_rx: Receiver<LogPacket>,
    stop_flag: Arc<AtomicBool>,
) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend).unwrap();

    let mut logs: Vec<LogPacket> = Vec::new();
    let mut input = String::new();
    let mut scroll: u16 = 0;
    const MAX_LOG_LINES: u16 = 100;

    while !stop_flag.load(std::sync::atomic::Ordering::Relaxed) {
        if event::poll(Duration::from_millis(100))? {
            match event::read() {
                Ok(Event::Key(key)) => match key.code {
                    KeyCode::Char(c) => input.push(c),
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Enter => {
                        handle_command(&input);
                        input.clear();
                    }
                    KeyCode::Up => {
                        if scroll < logs.len() as u16 {
                            scroll += 1;
                        }
                    }
                    KeyCode::Down => {
                        if scroll > 0 {
                            scroll -= 1;
                        }
                    }
                    KeyCode::End => {
                        scroll = 0;
                    }
                    KeyCode::Home => {
                        scroll = MAX_LOG_LINES;
                    }
                    _ => {}
                }
                Ok(Event::Mouse(mouse_event)) => match mouse_event.kind {
                    event::MouseEventKind::ScrollUp => {
                        if scroll < logs.len() as u16 {
                            scroll += 1;
                        }
                    }
                    event::MouseEventKind::ScrollDown => {
                        if scroll > 0 {
                            scroll -= 1;
                        }
                    }
                    _ => {}
                }
                _ => {}
            }
        }

        while let Ok(log) = tui_rx.try_recv() {
            logs.push(log);
            if logs.len() > MAX_LOG_LINES as usize {
                logs.remove(0);
            }
            if scroll != 0 {
                scroll += 1;
            }
        }

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
                .split(f.size());

            let log_lines = Text::from(logs.iter().map(|lp| {
                Line::from(vec![
                    Span::from(format!("[{}] [", lp.timestamp().format("%H:%M:%S"))),
                    Span::styled(lp.severity().get_name(), lp.severity().get_style()),
                    Span::from(format!("] [{}] {}", lp.source().get_name(), lp.message()))
                ])
            }).collect_vec());

            let log_height = chunks[0].height.saturating_sub(2);
            let total_lines = logs.len() as u16;
            if log_height + scroll > total_lines {
                let delta = (log_height + scroll) - total_lines;
                scroll = scroll.saturating_sub(delta);
            }
            let scroll_offset = total_lines
                .saturating_sub(log_height)
                .saturating_sub(scroll);

            let logs_widget = Paragraph::new(log_lines)
                .block(Block::default().title("Logs").borders(Borders::ALL))
                .scroll((scroll_offset, 0));

            let input_widget = Paragraph::new(format!("> {}", input))
                .block(Block::default().title("Command").borders(Borders::ALL));

            f.render_widget(logs_widget, chunks[0]);

            if total_lines > log_height {
                let mut scrollbar_state = ScrollbarState::new((total_lines - log_height) as usize).position(scroll_offset as usize);
                let scrollbar = Scrollbar::default()
                    .orientation(ScrollbarOrientation::VerticalRight)
                    .thumb_style(Style::default().fg(Color::Gray));

                f.render_stateful_widget(scrollbar, chunks[0], &mut scrollbar_state);
            }
            
            f.render_widget(input_widget, chunks[1]);
        })?;
    }

    drop(tui_rx);
    log(TUI, Info, "Restoring terminal");

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}
