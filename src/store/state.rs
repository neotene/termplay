#[derive(Default, Clone)]
pub struct State {
    pub is_logged: bool,
    pub is_registering: bool,
    pub login: String,
    pub password: String,
    pub register_login: String,
    pub register_confirm_login: String,
    pub register_password: String,
    pub register_confirm_password: String,
    pub error_message: String,
    pub show_exit_confirmation: bool,
}

impl State {}
