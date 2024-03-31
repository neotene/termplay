use std::time::Duration;

use tokio::sync::{ broadcast, mpsc };

use crate::termination::{ Interrupted, Terminator };

use super::action::Action;
use super::state::State;

pub struct Store {
    state_sender: mpsc::UnboundedSender<State>,
}

pub const NEW_LINE: &[u8; 2] = b"\r\n";

impl Store {
    pub fn new() -> (Self, mpsc::UnboundedReceiver<State>) {
        let (state_sender, state_receiver) = mpsc::unbounded_channel();

        (Self { state_sender }, state_receiver)
    }

    pub async fn do_loop(
        &self,
        mut terminator: Terminator,
        mut action_receiver: mpsc::UnboundedReceiver<Action>,
        _interrupt_receiver: broadcast::Receiver<Interrupted>
    ) -> anyhow::Result<Interrupted> {
        let mut state = State::default();

        self.state_sender.send(state.clone())?;

        let _ticker = tokio::time::interval(Duration::from_secs(1));

        // let mut opt_server_handle: Option<ServerHandle> = None;

        let result = loop {
            // let mut socket = opt_server_handle.unwrap();
            let buffer = vec![0; 1024];
            // SSLStream::poll_read(socket.as_mut(, ));
            // socket.
            tokio::select! {
                    Some(action) = action_receiver.recv() => match action {
                        Action::None => {
                        },
                        Action::Login => {
                        },
                        Action::ShowRegister => {
                            state.is_registering = true;
                            self.state_sender.send(state.clone())?;
                        },
                        Action::Register => {

                        },
                        Action::PreExit => {
                            state.show_exit_confirmation = true;
                            self.state_sender.send(state.clone())?;
                        },
                        Action::CancelExit => {
                            state.show_exit_confirmation = false;
                            self.state_sender.send(state.clone())?;
                        },
                        Action::Exit => {
                            terminator.terminate(Interrupted::UserInt)?;
                            break Interrupted::UserInt;
                        },
                    }
                    // _ = _ticker.tick() => {
                    //     // do something
                    // };
                }
        };
        Ok(result)
    }
}
