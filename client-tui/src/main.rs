pub mod blocking_widget;
mod choose_login;
mod login;

use crate::login::Login;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use h_mail_client::HResult;
use ratatui::DefaultTerminal;
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Paragraph};

fn main() -> HResult<()> {
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

pub enum KeyHandleType {
    Handled,
    NotHandled,
    ChangeState(State),
}

#[derive(Debug)]
pub enum State {
    Inbox,
    ChooseLogin,
    Login,
}

impl State {
    pub fn is_login(&self) -> bool {
        matches!(&self, State::Login)
    }
}

#[derive(Debug)]
pub struct App {
    running: bool,
    state: State,
    login: Login,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        App {
            running: false,
            state: State::Inbox,
            login: Login::new(),
        }
    }

    fn change_state(&mut self, new_state: State) {
        self.state = new_state;
        match self.state {
            State::Inbox => {}
            State::ChooseLogin => todo!(),
            State::Login => {
                self.login.on_enter();
            }
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> HResult<()> {
        self.running = true;

        self.change_state(State::ChooseLogin);

        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let title = Line::from(" H-Mail Client ").bold().blue().centered();

        let text = "";
        frame.render_widget(
            Paragraph::new(text)
                .block(
                    Block::bordered()
                        .border_type(BorderType::Double)
                        .title(title),
                )
                .centered(),
            frame.area(),
        );

        let rect = frame.area().inner(Margin::new(1, 1));

        match self.state {
            State::Inbox => {}
            State::ChooseLogin => todo!(),
            State::Login => {
                self.login.render(frame, rect);
            }
        }
    }

    fn handle_crossterm_events(&mut self) -> HResult<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match match self.state {
            State::Inbox => KeyHandleType::NotHandled,
            State::ChooseLogin => todo!(),
            State::Login => self.login.on_key_event(key),
        } {
            KeyHandleType::Handled => {
                return;
            }
            KeyHandleType::NotHandled => {}
            KeyHandleType::ChangeState(s) => {
                self.change_state(s);
                return;
            }
        }

        match (key.modifiers, key.code) {
            (_, KeyCode::Esc)
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
