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
}

impl Application {
    fn get_active_page_component(&self) -> &dyn UIObject<()> {
        match self.props.active_page {
            ActivePage::LoginPage => &self.login_page,
            ActivePage::RegisterPage => &self.register_page,
        }
    }

    fn get_active_page_component_mut(&mut self) -> &mut dyn UIObject {
        match self.props.active_page {
            ActivePage::LoginPage => &mut self.login_page,
            ActivePage::RegisterPage => &mut self.register_page,
        }
    }
}

impl UIObject<()> for Application {
    fn new(state: &State, action_sender: UnboundedSender<Action>, _: ()) -> Self {
        Self {
            login_page: LoginPage::new(state, action_sender.clone(), ()),
            register_page: RegisterPage::new(state, action_sender.clone(), ()),
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
