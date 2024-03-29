use ratatui::{ backend::Backend, Frame };
use tokio::sync::mpsc::UnboundedSender;

use crate::store::{ action::Action, state::State };

pub trait UIObject<InitProperties> {
    fn new(
        state: &State,
        action_sender: UnboundedSender<Action>,
        init_properties: InitProperties
    ) -> Self
        where Self: Sized;
    fn move_with_state(self, state: &State) -> Self where Self: Sized;
    fn handle_key_event(&mut self, event: crossterm::event::Event);
}

pub trait UIRender<Properties> {
    fn render(&self, frame: &mut Frame, properties: Properties);
}
