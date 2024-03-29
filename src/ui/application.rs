use ratatui::{ backend::Backend, Frame };
use tokio::sync::mpsc::UnboundedSender;

use crate::store::{ action::Action, state::State };

use super::{ login_page::LoginPage, ui_object::{ UIObject, UIRender } };

pub struct Application {
    login_page: LoginPage,
}

impl UIObject<()> for Application {
    fn new(state: &State, action_sender: UnboundedSender<Action>, _: ()) -> Self {
        Self {
            login_page: LoginPage::new(state, action_sender, ()),
        }
    }

    fn handle_key_event(&mut self, event: crossterm::event::Event) {
        self.login_page.handle_key_event(event);
    }
}

impl UIRender<()> for Application {
    fn render<B: Backend>(&self, frame: &mut Frame<B>, properties: ()) {
        self.login_page.render(frame, properties);
    }
}
