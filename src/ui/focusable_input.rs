use std::sync::Arc;

use crossterm::event::Event;

/// A trait for widgets that can handle input events and generate messages.
pub trait InputHandler {
    type Message;

    fn handle_input(&mut self, event: Event) -> Option<Self::Message>;
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
    mapper: Arc<dyn Fn(A::Message) -> Option<B> + Send + Sync>,
}

impl<A: InputHandler + 'static, B> MessageTranslator<A, B> {
    pub fn new(
        input_handler: A,
        mapper: impl Fn(A::Message) -> Option<B> + Send + Sync + 'static,
    ) -> Self {
        Self {
            input_handler,
            mapper: Arc::new(mapper),
        }
    }

    pub fn input_handler_mut(&mut self) -> &mut A {
        &mut self.input_handler
    }
}

impl<A: InputHandler, B> InputHandler for MessageTranslator<A, B> {
    type Message = B;

    fn handle_input(&mut self, event: Event) -> Option<Self::Message> {
        let msg = self.input_handler.handle_input(event);
        msg.and_then(|msg| (self.mapper)(msg))
    }
}
