trait UiObject {
    fn new(state: &State, action_sender: UnboundedSender<Action>) -> Self;
    fn handle_key_event(&self, event: crossterm::event::Event);
}
