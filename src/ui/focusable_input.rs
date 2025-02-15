use std::{error::Error, rc::Rc, time::Duration};

use crossterm::event;

/// A trait for widgets that can handle input events and generate messages.
/// If the widget needs to do manual handling of CrossTerm events, [`get_next_event`] is
/// provided to get the next event from the event queue.
pub trait InputHandler {
    type Message;

    /// Handle input. May return `None` if the input was handled internally.
    fn handle_input(&mut self) -> Result<Option<Self::Message>, Box<dyn Error>>;

    /// Get the next event from the event queue.
    fn get_next_event(&mut self) -> Result<Option<event::Event>, Box<dyn Error>> {
        let has_event = event::poll(Duration::from_millis(50))?;
        if !has_event {
            return Ok(None);
        }

        let event = event::read()?;
        Ok(Some(event))
    }
}

/// An [`InputHandler`] that wraps another `InputHandler` and translates messages it generates
/// from `A::Message` to `B`.
///
/// # Example:
///
/// ```rust
/// // `TextInputState` is an `InputHandler` that generates `TextInputMsg` messages.
/// let text_input_state = MessageTranslator::new(TextInputState::default(), |msg| match msg {
///     TextInputMsg::Close => Some(AppAction::CloseSearch),
///     TextInputMsg::Accept(input) => Some(AppAction::AcceptSearch(input)),
/// });
///
/// let msg = text_input_state.handle_input(Event::Key(KeyCode::Enter));
/// assert_eq!(msg, Some(AppAction::AcceptSearch("hello".to_string())));
/// ```
pub struct MessageTranslator<A: InputHandler, B> {
    input_handler: A,
    mapper: Rc<dyn Fn(A::Message) -> Option<B> + Send + Sync>,
}

impl<A: InputHandler + 'static, B> MessageTranslator<A, B> {
    pub fn new(
        input_handler: A,
        mapper: impl Fn(A::Message) -> Option<B> + Send + Sync + 'static,
    ) -> Self {
        Self {
            input_handler,
            mapper: Rc::new(mapper),
        }
    }

    pub fn input_handler_mut(&mut self) -> &mut A {
        &mut self.input_handler
    }
}

impl<A: InputHandler, B> InputHandler for MessageTranslator<A, B> {
    type Message = B;

    fn handle_input(&mut self) -> Result<Option<Self::Message>, Box<dyn Error>> {
        let msg = self.input_handler.handle_input()?;
        Ok(msg.and_then(|msg| (self.mapper)(msg)))
    }
}
