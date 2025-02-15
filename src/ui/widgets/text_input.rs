use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Text,
    widgets::{Block, Borders, Clear, Paragraph, StatefulWidget, Widget, Wrap},
};

use crate::ui::focusable_input::InputHandler;

#[derive(Debug, Clone)]
pub enum TextInputMsg {
    Close,
    Accept(String),
}

#[derive(Debug, Clone, Default)]
pub struct TextInputState {
    pub input: String,
    pub character_index: u16,
    pub cursor_position: (u16, u16),
}

impl TextInputState {
    pub fn from_str(s: &str) -> Self {
        Self {
            input: s.to_string(),
            character_index: s.len() as u16,
            ..Default::default()
        }
    }

    pub fn add_char(&mut self, c: char) {
        let pos = self.character_index as usize;
        self.input.insert(pos, c);
        self.character_index += 1;
    }

    pub fn backspace(&mut self) {
        if self.character_index > 0 {
            let pos = self.character_index as usize - 1;
            self.input.remove(pos);
            self.character_index -= 1;
        }
    }

    pub fn backspace_word(&mut self) {
        if self.character_index > 0 {
            let mut pos = self.character_index as usize - 1;

            while pos > 0
                && !self.input[pos - 1..pos]
                    .chars()
                    .next()
                    .unwrap()
                    .is_whitespace()
                && !matches!(
                    self.input[pos - 1..pos].chars().next().unwrap(),
                    '-' | '_'
                        | '+'
                        | '='
                        | ','
                        | '.'
                        | '/'
                        | '\\'
                        | ':'
                        | ';'
                        | '!'
                        | '?'
                        | '@'
                        | '#'
                        | '$'
                        | '%'
                        | '^'
                        | '&'
                        | '*'
                        | '('
                        | ')'
                        | '['
                        | ']'
                        | '{'
                        | '}'
                )
            {
                pos -= 1;
            }

            while pos < self.character_index as usize {
                self.input.remove(pos);
                self.character_index -= 1;
            }
        }
    }

    pub fn move_left(&mut self) {
        if self.character_index > 0 {
            self.character_index -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.character_index < self.input.len() as u16 {
            self.character_index += 1;
        }
    }
}

impl InputHandler for TextInputState {
    type Message = TextInputMsg;

    fn handle_input(&mut self, event: Event) -> Option<TextInputMsg> {
        if let Event::Key(key) = event {
            let modifiers = key.modifiers;
            let ctrl = modifiers.contains(event::KeyModifiers::CONTROL);

            match key.code {
                KeyCode::Esc => Some(TextInputMsg::Close),
                KeyCode::Backspace => {
                    self.backspace();
                    None
                }
                KeyCode::Left => {
                    self.move_left();
                    None
                }
                KeyCode::Right => {
                    self.move_right();
                    None
                }
                KeyCode::Enter => Some(TextInputMsg::Accept(self.input.clone())),
                KeyCode::Char('c') if ctrl => Some(TextInputMsg::Close),
                KeyCode::Char('w') if ctrl => {
                    self.backspace_word();
                    None
                }
                KeyCode::Char('a') if ctrl => {
                    self.character_index = 0;
                    None
                }
                KeyCode::Char('e') if ctrl => {
                    self.character_index = self.input.len() as u16;
                    None
                }
                KeyCode::Char(c) => {
                    self.add_char(c);
                    None
                }
                _ => None,
            }
        } else {
            None
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
        extended_by_one.y = extended_by_one.y.saturating_sub(1);
        extended_by_one.width += 1;
        extended_by_one.height += 1;

        Clear.render(extended_by_one, buf);
        let search_block = Block::default()
            .title(self.title.as_deref().unwrap_or(""))
            .borders(Borders::ALL);
        search_block.render(area, buf);

        let text_area = Rect::new(area.x + 2, area.y + 1, area.width - 2, 1);
        let search_text =
            Paragraph::new(Text::from(state.input.clone())).wrap(Wrap { trim: false });

        search_text.render(text_area, buf);
        let cursor_position = (text_area.x + state.character_index, text_area.y);
        state.cursor_position = cursor_position;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_state() {
        let mut state = TextInputState::default();
        state.add_char('a');
        state.add_char('b');
        state.add_char('c');
        assert_eq!(state.input, "abc");
        assert_eq!(state.character_index, 3);
        state.backspace();
        assert_eq!(state.input, "ab");
        state.add_char('d');
        assert_eq!(state.input, "abd");
        state.move_left();
        assert_eq!(state.input, "abd");
        assert_eq!(state.character_index, 2);
        state.backspace();
        assert_eq!(state.input, "ad");
        state.add_char('e');
        assert_eq!(state.input, "aed");
    }
}
