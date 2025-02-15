mod line_buffer;

use crate::{
    state::line_buffer::LogBuffer,
    ui::{
        focusable_input::{InputHandler, MessageTranslator},
        widgets::text_input::{TextInputMsg, TextInputState},
    },
};

use std::{rc::Rc, sync::Mutex};

type MessageTranslatorRc<T> = Rc<Mutex<MessageTranslator<T, AppAction>>>;
type InputHandlerRc = Rc<Mutex<dyn InputHandler<Message = AppAction>>>;

#[derive(Debug, Clone)]
pub enum AppAction {
    ScrollUp(u16),
    ScrollDown(u16),
    ScrollToLine(u16),
    AttachToBottom,
    ToggleLineNumbers,
    OpenSearch,
    CloseSearch,
    AcceptSearch(String),
    OpenGoToLine,
    CloseGoToLine,
    AcceptGoToLine(String),
    Quit,
}

pub struct AppState {
    pub lines: LogBuffer,
    pub line_num: u16,
    pub attached_to_bottom: bool,
    pub search_input: Option<MessageTranslatorRc<TextInputState>>,
    pub search: Option<String>,
    pub go_to_line: Option<MessageTranslatorRc<TextInputState>>,
    pub show_line_numbers: bool,

    pub focused_input: Option<InputHandlerRc>,
    pub last_frame_height: u16,
    pub quit: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            lines: LogBuffer::new(),
            line_num: 1,
            attached_to_bottom: false,
            search_input: None,
            search: None,
            go_to_line: None,
            show_line_numbers: true,

            focused_input: None,
            last_frame_height: 0,
            quit: false,
        }
    }

    pub fn handle_action(&mut self, action: AppAction) {
        match action {
            AppAction::ScrollUp(amount) => self.scroll_up_by(amount),
            AppAction::ScrollDown(amount) => self.scroll_down_by(amount),
            AppAction::ScrollToLine(line_number) => self.set_line_number(line_number),
            AppAction::AttachToBottom => self.attach_to_bottom(),
            AppAction::ToggleLineNumbers => self.toggle_line_numbers(),
            AppAction::OpenSearch => self.open_search(),
            AppAction::CloseSearch => self.close_search(),
            AppAction::OpenGoToLine => self.open_go_to_line(),
            AppAction::CloseGoToLine => self.close_go_to_line(),
            AppAction::Quit => self.quit = true,
            AppAction::AcceptSearch(query) => {
                self.search = Some(query);
                self.close_search();
            }
            AppAction::AcceptGoToLine(line_number) => {
                if let Ok(line_number) = line_number.parse::<u16>() {
                    self.set_line_number(line_number);
                    self.close_go_to_line();
                }
            }
        }
    }

    pub fn add_line(&mut self, line: String) {
        self.lines.add_line(line);
    }

    pub fn get_lines(&self) -> Vec<String> {
        self.lines.get_lines(self.line_num, self.last_frame_height)
    }

    pub fn scroll_up_by(&mut self, amount: u16) {
        self.attached_to_bottom = false;
        if amount >= self.line_num {
            self.line_num = 1;
        } else {
            self.line_num -= amount;
        }
    }

    pub fn scroll_down_by(&mut self, amount: u16) {
        self.attached_to_bottom = false;
        if self.line_num + amount > self.lines.len() as u16 - 1 {
            self.line_num = self.lines.len() as u16 - 1;
        } else {
            self.line_num += amount;
        }
    }

    pub fn attach_to_bottom(&mut self) {
        self.attached_to_bottom = true;
        let last_line_number = self.lines.len() as u16;
        if last_line_number > self.last_frame_height {
            self.line_num = last_line_number - self.last_frame_height;
        }
    }

    pub fn set_line_number(&mut self, line_number: u16) {
        if line_number > self.lines.len() as u16 {
            self.line_num = self.lines.len() as u16 - 1;
        } else {
            self.line_num = line_number;
        }
    }

    pub fn toggle_line_numbers(&mut self) {
        self.show_line_numbers = !self.show_line_numbers;
    }

    pub fn open_search(&mut self) {
        let search_input = self
            .search
            .as_ref()
            .map(|s| TextInputState::from_str(s))
            .unwrap_or_default();

        let search_input = MessageTranslator::new(search_input, {
            |msg| match msg {
                TextInputMsg::Close => Some(AppAction::CloseSearch),
                TextInputMsg::Accept(input) => Some(AppAction::AcceptSearch(input)),
            }
        });

        let search_input = Rc::new(Mutex::new(search_input));
        self.search_input = Some(search_input.clone());
        self.focused_input = Some(search_input);
    }

    pub fn close_search(&mut self) {
        self.search_input = None;
        self.focused_input = None;
    }

    pub fn open_go_to_line(&mut self) {
        let go_to_line = MessageTranslator::new(TextInputState::default(), |msg| match msg {
            TextInputMsg::Close => Some(AppAction::CloseGoToLine),
            TextInputMsg::Accept(input) => Some(AppAction::AcceptGoToLine(input)),
        });

        let go_to_line = Rc::new(Mutex::new(go_to_line));
        self.go_to_line = Some(go_to_line.clone());
        self.focused_input = Some(go_to_line);
    }

    pub fn close_go_to_line(&mut self) {
        self.go_to_line = None;
        self.focused_input = None;
    }
}
