use ratatui::{ backend::Backend, Frame };
use tokio::sync::mpsc::UnboundedSender;

use crate::store::{ action::Action, state::State };

use super::{
    login_page::LoginPage,
    register_page::RegisterPage,
    ui_object::{ UIObject, UIRender },
};

enum ActivePage {
    LoginPage,
    RegisterPage,
}

pub struct Application {
    login_page: LoginPage,
    register_page: RegisterPage,
    active_page: ActivePage,
}

impl UIObject<()> for Application {
    fn new(state: &State, action_sender: UnboundedSender<Action>, _: ()) -> Self {
        Self {
            login_page: LoginPage::new(state, action_sender.clone(), ()),
            register_page: RegisterPage::new(state, action_sender.clone(), ()),
            active_page: ActivePage::LoginPage,
        }
    }

    fn move_with_state(self, state: &State) -> Self {
        Self {
            login_page: self.login_page.move_with_state(state),
            register_page: self.register_page.move_with_state(state),
            active_page: match state.is_registering {
                false => ActivePage::LoginPage,
                true => ActivePage::RegisterPage,
            },
        }
    }

    fn handle_key_event(&mut self, event: crossterm::event::Event) {
        match self.active_page {
            ActivePage::LoginPage => self.login_page.handle_key_event(event),
            ActivePage::RegisterPage => self.register_page.handle_key_event(event),
        }
    }
}

impl UIRender<()> for Application {
    fn render<B: Backend>(&self, frame: &mut Frame<B>, properties: ()) {
        match self.active_page {
            ActivePage::LoginPage => self.login_page.render(frame, properties),
            ActivePage::RegisterPage => self.register_page.render(frame, properties),
        }
    }
}
