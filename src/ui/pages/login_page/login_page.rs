use ratatui::layout::Rect;
use ratatui::Frame;
use tokio::sync::mpsc::UnboundedSender;
use tui_textarea::TextArea;

use crate::store::action::Action;
use crate::store::state::State;
use crate::ui::pages::widgets::text_input::{ self, TextInput };
use crate::ui::ui_object::ui_object::{ UiObject, UiRender };

pub struct LoginPage {
    login_field: TextInput,
}

impl UiObject for LoginPage {
    fn new(state: &State, action_sender: UnboundedSender<Action>) -> Self {
        Self {
            login_field: TextInput::new(state, action_sender),
        }
    }

    fn handle_key_event(&mut self, event: crossterm::event::Event) {}
}

impl UiRender<()> for LoginPage {
    fn render(&self, frame: &mut Frame, _properties: ()) {
        self.login_field.render(frame, text_input::RenderProperties {
            title: String::from("Login"),
            area: Rect::new(0, 0, 20, 1),
            border_color: ratatui::style::Color::White,
        });
    }
}
