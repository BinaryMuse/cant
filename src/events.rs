use std::{error::Error, rc::Rc, sync::RwLock, time::Duration};

use crossterm::event::{self, Event, KeyCode};

use crate::{state::AppAction, AppState};

pub fn poll_events(state: Rc<RwLock<AppState>>) -> Result<Option<AppAction>, Box<dyn Error>> {
    let has_event = event::poll(Duration::from_millis(50))?;
    if !has_event {
        return Ok(None);
    }

    let next = event::read()?;

    let state = state.read().unwrap();
    if let Some(focused) = state.focused_input.as_ref() {
        let action = focused.lock().unwrap().handle_input(next);
        Ok(action)
    } else {
        Ok(handle_general_events(&state, &next))
    }
}

fn handle_general_events(state: &AppState, next: &Event) -> Option<AppAction> {
    if let Event::Key(key) = next {
        match key.code {
            KeyCode::Down => Some(AppAction::ScrollDown(1)),
            KeyCode::Up => Some(AppAction::ScrollUp(1)),
            KeyCode::Esc => Some(AppAction::Quit),
            KeyCode::Char('q') => Some(AppAction::Quit),
            KeyCode::Char('t') => Some(AppAction::ScrollToLine(1)),
            KeyCode::Char(' ') => {
                let lines = state.last_frame_height;
                Some(AppAction::ScrollDown(lines / 2))
            }
            KeyCode::Char('b') => Some(AppAction::AttachToBottom),
            KeyCode::Char('N') => Some(AppAction::ToggleLineNumbers),
            KeyCode::Char('g') => Some(AppAction::OpenGoToLine),
            KeyCode::Char('/') => Some(AppAction::OpenSearch),
            _ => None,
        }
    } else {
        None
    }
}
