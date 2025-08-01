use crate::KeyHandleType;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::prelude::Line;
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use crate::blocking_widget::BlockingWidgetTrait;

pub struct PreAuth;
impl BlockingWidgetTrait<Option<String>> for PreAuth {
    fn render_preblock(&mut self, frame: &mut Frame, area: Rect) {
        todo!()
    }

    fn run_task_blocking() -> Option<String> {
        // Handle::current().block_on(check_auth())
        todo!()
    }
}

#[derive(Debug)]
pub struct Login {
    username: String,
    password: String,
    address: String,
    create_account: bool,
    focus: usize,
}

impl Login {
    pub fn new() -> Self {
        Login {
            username: "".to_string(),
            password: "".to_string(),
            address: "".to_string(),
            create_account: false,
            focus: 0,
        }
    }

    pub fn on_enter(&mut self) {
        self.username.clear();
        self.password.clear();
        self.address.clear();
        self.create_account = false;
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let box_width = 50;
        let box_height = 13;

        let x = area.x + (area.width.saturating_sub(box_width)) / 2;
        let y = area.y + (area.height.saturating_sub(box_height)) / 2;
        let inner_area = Rect::new(x, y, box_width, box_height);

        let outer_block = Block::default()
            .title(" Login ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        frame.render_widget(outer_block, inner_area);

        let inner_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(2),
            ])
            .split(inner_area);

        let highlight_style = Style::default().fg(Color::Yellow);

        let username = if self.focus == 0 {
            Line::from(format!("> Username: {}", self.username)).style(highlight_style)
        } else {
            Line::from(format!("  Username: {}", self.username))
        };
        let password = if self.focus == 1 {
            Line::from(format!("> Password: {}", "*".repeat(self.password.len())))
                .style(highlight_style)
        } else {
            Line::from(format!("  Password: {}", "*".repeat(self.password.len())))
        };
        let address = if self.focus == 2 {
            Line::from(format!("> Server Address: {}", self.address)).style(highlight_style)
        } else {
            Line::from(format!("  Server Address: {}", self.address))
        };
        let create = if self.focus == 3 {
            Line::from(format!(
                "> Create Account: {}",
                if self.create_account { "Yes" } else { "No" }
            ))
            .style(highlight_style)
        } else {
            Line::from(format!(
                "  Create Account: {}",
                if self.create_account { "Yes" } else { "No" }
            ))
        };
        let login = if self.focus == 4 {
            Line::from("> [Login]").style(highlight_style)
        } else {
            Line::from("  [Login]")
        };

        frame.render_widget(Paragraph::new(username), inner_chunks[1]);
        frame.render_widget(Paragraph::new(password), inner_chunks[2]);
        frame.render_widget(Paragraph::new(address), inner_chunks[3]);
        frame.render_widget(Paragraph::new(create), inner_chunks[4]);
        frame.render_widget(Paragraph::new(login), inner_chunks[5]);
    }

    pub fn on_key_event(&mut self, key: KeyEvent) -> KeyHandleType {
        if key.kind != KeyEventKind::Press || key.modifiers.contains(KeyModifiers::CONTROL) {
            return KeyHandleType::NotHandled;
        }

        match key.code {
            KeyCode::Tab | KeyCode::Down => {
                self.focus = (self.focus + 1) % 5;
                return KeyHandleType::Handled;
            }
            KeyCode::BackTab | KeyCode::Up => {
                self.focus = (self.focus + 4) % 5;
                return KeyHandleType::Handled;
            }
            KeyCode::Char(c) => {
                match self.focus {
                    0 => self.username.push(c),
                    1 => self.password.push(c),
                    2 => self.address.push(c),
                    _ => {}
                }
                return KeyHandleType::Handled;
            }
            KeyCode::Backspace => {
                match self.focus {
                    0 => {
                        self.username.pop();
                    }
                    1 => {
                        self.password.pop();
                    }
                    2 => {
                        self.address.pop();
                    }
                    _ => {}
                }
                return KeyHandleType::Handled;
            }
            KeyCode::Enter => match self.focus {
                3 => {
                    self.create_account = !self.create_account;
                    return KeyHandleType::Handled;
                }
                4 => {}
                _ => {}
            },
            _ => {}
        }

        KeyHandleType::NotHandled
    }
}
