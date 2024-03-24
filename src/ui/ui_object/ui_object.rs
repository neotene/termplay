use ratatui::{ backend::Backend, Frame };
use tokio::sync::mpsc::UnboundedSender;

use crate::store::{ action::Action, state::State };

pub trait UIObject {
    fn new(state: &State, action_sender: UnboundedSender<Action>) -> Self;
    fn handle_key_event(&mut self, event: crossterm::event::Event);
}

pub trait UIRender<Properties> {
    fn render<B: Backend>(&self, frame: &mut Frame<B>, properties: Properties);
}
