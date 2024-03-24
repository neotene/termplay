use tokio::sync::mpsc::UnboundedSender;

use crate::store::{ action::Action, state::State };

use super::{ pages::login_page::login_page::LoginPage, ui_object::ui_object::UiObject };

pub struct Application {
    login_page: LoginPage,
}

impl UiObject for Application {
    fn new(state: &State, action_sender: UnboundedSender<Action>) -> Self {
        Self {
            login_page: LoginPage::new(state, action_sender),
        }
    }

    fn handle_key_event(&mut self, event: crossterm::event::Event) {
        self.login_page.handle_key_event(event);
    }
}
