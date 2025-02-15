use std::{
    rc::Rc,
    sync::RwLock,
};

use ratatui::{
    layout::Flex,
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use widgets::text_input::TextInput;

use crate::AppState;

pub mod focusable_input;
pub mod widgets;

pub fn render(frame: &mut Frame, state: &Rc<RwLock<AppState>>) {
    let outer_block = Block::default().title("logfile").borders(Borders::ALL);
    let size = outer_block.inner(frame.area());
    state.write().unwrap().last_frame_height = size.height;

    let lines = &state.read().unwrap().get_lines();

    let start_line_no = state.read().unwrap().line_num;
    let to_line_no = start_line_no + lines.len() as u16;
    let line_no_length = to_line_no.checked_ilog10().unwrap_or(0) + 1;

    let left_col_width = if state.read().unwrap().show_line_numbers {
        line_no_length as u16 + 2
    } else {
        0
    };

    let outer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Length(left_col_width),
            Constraint::Percentage(100),
        ])
        .split(frame.area());

    if left_col_width > 0 {
        let mut left_frame = outer_layout[0];
        left_frame.y += 1;
        left_frame.height -= 1;

        let line_numbers = (start_line_no..to_line_no)
            .map(|n| format!(" {:>width$}", n, width = line_no_length as usize))
            .collect::<Vec<_>>();

        let line_numbers = Paragraph::new(Text::from(line_numbers.join("\n")));

        frame.render_widget(line_numbers, left_frame);
    }

    let log_text = Paragraph::new(Text::from(lines.join("\n"))).block(outer_block);

    let right_frame = outer_layout[1];
    frame.render_widget(log_text, right_frame);

    let state = state.read().unwrap();
    if let Some(search) = state.search_input.as_ref() {
        let center = center_inside(frame.area(), frame.area().width - 16, 3);
        let text_input = TextInput::default().titled("Search");
        let mut translator = search.lock().unwrap();
        let input_handler = translator.input_handler_mut();

        frame.render_stateful_widget(text_input, center, input_handler);
        frame.set_cursor_position((
            input_handler.cursor_position.0,
            input_handler.cursor_position.1,
        ));
    } else if let Some(go_to_line) = state.go_to_line.as_ref() {
        let center = center_inside(frame.area(), frame.area().width / 2, 3);
        let text_input = TextInput::default().titled("Line #");
        let mut translator = go_to_line.lock().unwrap();
        let input_handler = translator.input_handler_mut();

        frame.render_stateful_widget(text_input, center, input_handler);
        frame.set_cursor_position((
            input_handler.cursor_position.0,
            input_handler.cursor_position.1,
        ));
    }
}

fn center_inside(area: Rect, width: u16, height: u16) -> Rect {
    let [center] = Layout::horizontal(vec![Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(area);
    let [center] = Layout::vertical(vec![Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(center);

    center
}
