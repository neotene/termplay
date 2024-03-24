struct LoginPage {}

impl UiObject for LoginPage {
    fn new(state: &State, action_sender: UnboundedSender<Action>) -> Self {
        Self {}
    }

    fn handle_key_event(&self, event: crossterm::event::Event) {}
}
