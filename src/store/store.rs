use native_tls::TlsConnector;
use tokio::sync::{ broadcast, mpsc };
use tokio_native_tls::native_tls;
use std::error::Error;
use std::net::ToSocketAddrs;
use std::pin::Pin;
use std::time::Duration;
use tokio::io::{ AsyncRead, AsyncReadExt, AsyncWriteExt, ReadBuf };
use tokio::net::TcpStream;

use crate::termination::{ Interrupted, Terminator };

use super::action::Action;
use super::command;
use super::state::State;

pub type SSLStream = tokio_native_tls::TlsStream<TcpStream>;

pub struct Store {
    state_sender: mpsc::UnboundedSender<State>,
}

pub struct CommandWriter {
    socket: SSLStream,
}

pub const NEW_LINE: &[u8; 2] = b"\r\n";

impl CommandWriter {
    pub fn new(writer: SSLStream) -> Self {
        Self { socket: writer }
    }

    pub async fn write(&mut self, command: &command::UserCommand) -> anyhow::Result<()> {
        let mut serialized_bytes = serde_json::to_vec(command)?;
        serialized_bytes.extend_from_slice(NEW_LINE);

        self.socket.write_all(serialized_bytes.as_slice()).await?;

        Ok(())
    }
}

type ServerHandle = Pin<Box<SSLStream>>;

// pub type BoxedStream<Item> = Pin<Box<dyn SSLStream<Item = Item> + Send>>;

// pub type EventStream = BoxedStream<anyhow::Result<event::Event>>;

async fn create_server_handle() -> anyhow::Result<ServerHandle> {
    let addr = "termplay.xyz:443".to_socket_addrs()?.next().unwrap();

    let socket = TcpStream::connect(&addr).await?;
    let cx = TlsConnector::builder().build()?;
    let cx = tokio_native_tls::TlsConnector::from(cx);

    let socket = cx.connect("termplay.xyz", socket).await?;

    Ok(Pin::new(Box::new(socket)))
}

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
            let mut socket = opt_server_handle.unwrap();
            let buffer = vec![0; 1024];
            SSLStream::poll_read(socket.as_mut(, ));
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
                    _ = _ticker.tick() => {
                        // do something
                    };
                }
        };
        Ok(result)
    }
}
