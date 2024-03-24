use tokio::sync::{ broadcast, mpsc::{ self, UnboundedReceiver } };

use crate::{ store::action::Action, termination::Interrupted };

pub struct UI {
    action_sender: mpsc::UnboundedSender<Action>,
}

impl UI {
    pub fn new() -> (Self, UnboundedReceiver<Action>) {
        let (action_sender, action_receiver) = mpsc::unbounded_channel();

        (Self { action_sender }, action_receiver)
    }
    pub async fn do_loop(
        &self,
        _state_receiver: UnboundedReceiver<crate::store::state::State>,
        _interrupt_receiver: broadcast::Receiver<Interrupted>
    ) -> anyhow::Result<Interrupted> {
        Ok(Interrupted::UserInt)
    }
}
