use std::time::Duration;

use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

use tokio::sync::{ broadcast, mpsc };

use std::pin::Pin;

use crate::termination::{ Interrupted, Terminator };

use super::action::Action;
use super::state::State;

pub struct Store {
    state_sender: mpsc::UnboundedSender<State>,
}

pub type SSLStream = tokio_native_tls::TlsStream<TcpStream>;

type ServerHandle = Pin<Box<SSLStream>>;

// pub const NEW_LINE: &[u8; 2] = b"\r\n";

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

        let mut opt_server_handle: Option<ServerHandle> = None;

        let result = loop {
            if let Some(server) = opt_server_handle.as_mut() {
                let mut buffer = [0; 10];
                tokio::select! {
                    data = server.read(&mut buffer) => match data {
                        Ok(0) => {
                            break Interrupted::UserInt;
                        },
                        Ok(n) => {
                            let data = &buffer[..n];
                            let data = std::str::from_utf8(data).unwrap();
                            println!("Received: {}", data);
                        },
                        Err(e) => {
                            eprintln!("Error reading from server: {}", e);
                            break Interrupted::UserInt;
                        }
                    },
                }
            } else {
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
                }
            }
        };
        Ok(result)
    }
}
