use tokio::sync::mpsc::UnboundedSender;

use crate::store::{ action::Action, state::State };

use super::{ button::Button, text_input::TextInput, ui_object::UIObject };

pub enum Focus {
    LoginField,
    ConfirmLoginField,
    PasswordField,
    ConfirmPasswordField,
    BackButton,
    RegisterButton,
}

const DEFAULT_HOVERED_SECTION: Focus = Focus::LoginField;

struct RegisterPage {
    login_field: TextInput,
    confirm_login_field: TextInput,
    password_field: TextInput,
    confirm_password_field: TextInput,
    back_button: Button,
    register_button: Button,
    last_hovered_section: Focus,
    active_section: Option<Focus>,
}

impl UIObject for RegisterPage {
    fn new(state: &State, action_sender: UnboundedSender<Action>, _: ()) -> Self {
        Self {
            login_field: TextInput::new(state, action_sender, ()),
            confirm_login_field: TextInput::new(state, action_sender, ()),
            password_field: TextInput::new(state, action_sender, ()),
            confirm_password_field: TextInput::new(state, action_sender, ()),
            back_button: Button::new(state, action_sender, ()),
            register_button: Button::new(state, action_sender, ()),
            last_hovered_section: DEFAULT_HOVERED_SECTION,
            active_section: None,
        }
    }

    fn handle_key_event(&mut self, event: crossterm::event::Event) {
        self.login_field.handle_key_event(event);
        self.confirm_login_field.handle_key_event(event);
        self.password_field.handle_key_event(event);
        self.confirm_password_field.handle_key_event(event);
        self.back_button.handle_key_event(event);
        self.register_button.handle_key_event(event);
    }
}
