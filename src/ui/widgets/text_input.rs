use std::{error::Error, rc::Rc};

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    text::Text,
    widgets::{Block, Borders, Clear, Paragraph, StatefulWidget, Widget, Wrap},
};
use tui_input::{Input, InputRequest};

use crate::ui::focusable_input::InputHandler;

#[derive(Debug, Clone)]
pub enum TextInputMsg {
    Close,
    Accept(String),
    Change(String),
}

#[derive(Debug, Clone, Default)]
pub struct TextInputState {
    pub input: Input,
    pub cursor_position: (u16, u16),
    pub color: Color,
}

impl TextInputState {
    pub fn from_str(s: &str) -> Self {
        Self {
            input: Input::new(s.to_string()),
            color: Color::Reset,
            ..Default::default()
        }
    }
}

impl InputHandler for TextInputState {
    type Message = TextInputMsg;

    fn handle_input(&mut self) -> Result<Option<TextInputMsg>, Box<dyn Error>> {
        let event = self.get_next_event()?;
        if event.is_none() {
            return Ok(None);
        }

        let event = event.unwrap();

        if let Event::Key(key) = event {
            let modifiers = key.modifiers;
            let ctrl = modifiers.contains(event::KeyModifiers::CONTROL);

            let msg = match key.code {
                KeyCode::Esc => Some(TextInputMsg::Close),
                KeyCode::Enter => Some(TextInputMsg::Accept(self.input.value().to_string())),
                KeyCode::Char('c') if ctrl => Some(TextInputMsg::Close),

                KeyCode::Backspace => {
                    self.input.handle(InputRequest::DeletePrevChar);
                    Some(TextInputMsg::Change(self.input.value().to_string()))
                }
                KeyCode::Char('h') if ctrl => {
                    self.input.handle(InputRequest::DeletePrevChar);
                    Some(TextInputMsg::Change(self.input.value().to_string()))
                }

                KeyCode::Delete => {
                    self.input.handle(InputRequest::DeleteNextChar);
                    Some(TextInputMsg::Change(self.input.value().to_string()))
                }
                KeyCode::Char('d') if ctrl => {
                    self.input.handle(InputRequest::DeleteNextChar);
                    Some(TextInputMsg::Change(self.input.value().to_string()))
                }

                KeyCode::Left => {
                    self.input.handle(InputRequest::GoToPrevChar);
                    None
                }
                KeyCode::Char('b') if ctrl => {
                    self.input.handle(InputRequest::GoToPrevChar);
                    None
                }

                KeyCode::Right => {
                    self.input.handle(InputRequest::GoToNextChar);
                    None
                }
                KeyCode::Char('f') if ctrl => {
                    self.input.handle(InputRequest::GoToNextChar);
                    None
                }

                KeyCode::Home => {
                    self.input.handle(InputRequest::GoToStart);
                    None
                }
                KeyCode::Char('a') if ctrl => {
                    self.input.handle(InputRequest::GoToStart);
                    None
                }

                KeyCode::End => {
                    self.input.handle(InputRequest::GoToEnd);
                    None
                }
                KeyCode::Char('e') if ctrl => {
                    self.input.handle(InputRequest::GoToEnd);
                    None
                }

                KeyCode::Char('u') if ctrl => {
                    while self.input.cursor() > 0 {
                        self.input.handle(InputRequest::DeletePrevChar);
                    }
                    Some(TextInputMsg::Change(self.input.value().to_string()))
                }
                KeyCode::Char('k') if ctrl => {
                    self.input.handle(InputRequest::DeleteTillEnd);
                    Some(TextInputMsg::Change(self.input.value().to_string()))
                }
                KeyCode::Char('w') if ctrl => {
                    // delete word leading to cursor
                    self.input.handle(InputRequest::DeletePrevWord);
                    Some(TextInputMsg::Change(self.input.value().to_string()))
                }

                KeyCode::Char(char) => {
                    self.input.handle(InputRequest::InsertChar(char));
                    Some(TextInputMsg::Change(self.input.value().to_string()))
                }
                _ => None,
            };

            Ok(msg)
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct TextInput {
    pub title: Option<String>,
}

impl TextInput {
    pub fn titled(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }
}

impl StatefulWidget for TextInput {
    type State = TextInputState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut extended_by_one = area;
        extended_by_one.x = extended_by_one.x.saturating_sub(1);
        extended_by_one.width += 2;

        Clear.render(extended_by_one, buf);
        let search_block = Block::default()
            .title(self.title.as_deref().unwrap_or(""))
            .borders(Borders::ALL);
        search_block.render(area, buf);

        let text_area = Rect::new(area.x + 2, area.y + 1, area.width - 2, 1);
        let search_text =
            Paragraph::new(Text::from(state.input.to_string().to_owned()).style(state.color))
                .wrap(Wrap { trim: false });

        search_text.render(text_area, buf);
        let cursor_position = (
            text_area.x + state.input.visual_cursor() as u16,
            text_area.y,
        );
        state.cursor_position = cursor_position;
    }
}
