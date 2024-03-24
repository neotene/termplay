use std::time::Duration;

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
        terminator: Terminator,
        action_receiver: mpsc::UnboundedReceiver<Action>,
        interrupt_receiver: broadcast::Receiver<Interrupted>
    ) -> anyhow::Result<Interrupted> {
        let mut state = State::default();

        self.state_sender.send(state.clone())?;

        let mut ticker = tokio::time::interval(Duration::from_secs(1));

        loop {
            tokio::select! {
                    Some(action) = action_receiver.recv() => match action {
                        Action::Login => {
                            print!("Logging in...");
                        },
                        Action::Register => {
                            println!("Registering...");
                        },
                        Action::Exit => {
                            break;
                        },
                    }
                }
        }
        Ok(Interrupted::UserInt)
    }
}
