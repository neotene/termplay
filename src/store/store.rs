use tokio::sync::{ broadcast, mpsc };

use state::State;

use crate::termination::{ Interrupted, Terminator };

use super::{ action::Action, state };

pub struct Store {
    state_sender: mpsc::UnboundedSender<State>,
}

impl Store {
    pub fn new() -> (Self, mpsc::UnboundedReceiver<State>) {
        let (state_sender, state_receiver) = mpsc::unbounded_channel();

        (Self { state_sender }, state_receiver)
    }

    pub async fn do_loop(
        &self,
        _terminator: Terminator,
        _action_receiver: mpsc::UnboundedReceiver<Action>,
        _interrupt_receiver: broadcast::Receiver<Interrupted>
    ) -> anyhow::Result<Interrupted> {
        Ok(Interrupted::UserInt)
    }
}
