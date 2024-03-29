use super::event;

#[derive(Default, Clone)]
pub enum ConnectionStatus {
    #[default]
    Idle,
    Connecting,
    Connected,
    Errored {
        message: String,
    },
}

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
    pub connection_status: ConnectionStatus,
}

impl State {
    pub fn handle_server_event(&mut self, event: &event::Event) {
        match event {
            event::Event::RegisterResponse(event::RegisterResponseEvent { success, message }) => {
                self.connection_status = ConnectionStatus::Idle;
                self.error_message = message.clone();
                if *success {
                    self.is_registering = false;
                }
            }
        }
    }
}
