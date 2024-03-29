use std::{ pin::Pin, time::Duration };

use anyhow::Context;
use tokio::{
    io::{ AsyncBufReadExt, AsyncWriteExt, BufReader },
    net::{ tcp::OwnedWriteHalf, TcpStream },
    sync::{ broadcast, mpsc },
};

use state::State;

use crate::{ store::state::ConnectionStatus, termination::{ Interrupted, Terminator } };

use super::{ action::Action, command, event, state };

pub struct Store {
    state_sender: mpsc::UnboundedSender<State>,
}

use tokio_stream::{ wrappers::LinesStream, Stream, StreamExt };

pub type BoxedStream<Item> = Pin<Box<dyn Stream<Item = Item> + Send>>;

pub type EventStream = BoxedStream<anyhow::Result<event::Event>>;

pub struct CommandWriter {
    writer: OwnedWriteHalf,
}

pub const NEW_LINE: &[u8; 2] = b"\r\n";

impl CommandWriter {
    pub fn new(writer: OwnedWriteHalf) -> Self {
        Self { writer }
    }

    pub async fn write(&mut self, command: &command::UserCommand) -> anyhow::Result<()> {
        let mut serialized_bytes = serde_json::to_vec(command)?;
        serialized_bytes.extend_from_slice(NEW_LINE);

        self.writer.write_all(serialized_bytes.as_slice()).await?;

        Ok(())
    }
}

pub fn split_tcp_stream(stream: TcpStream) -> (EventStream, CommandWriter) {
    let (reader, writer) = stream.into_split();

    (
        Box::pin(
            LinesStream::new(BufReader::new(reader).lines()).map(|line| {
                line.context("could not read line from the server").and_then(|line| {
                    serde_json
                        ::from_str::<event::Event>(&line)
                        .context("failed to deserialize event from the server")
                })
            })
        ),
        CommandWriter::new(writer),
    )
}

type ServerHandle = (EventStream, CommandWriter);

async fn create_server_handle(addr: &str) -> anyhow::Result<ServerHandle> {
    let stream = TcpStream::connect(addr).await?;
    let (event_stream, command_writer) = split_tcp_stream(stream);

    Ok((event_stream, command_writer))
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
            if let Some((event_stream, command_writer)) = opt_server_handle.as_mut() {
                tokio::select! {
                maybe_event = event_stream.next() => match maybe_event {
                    Some(Ok(event)) => {
                        state.handle_server_event(&event);
                    },
                    // server disconnected, we need to reset the state
                    None => {
                        opt_server_handle = None;
                        state = State::default();
                    },
                    _ => (),
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
                            state.connection_status = ConnectionStatus::Connecting;
                            // emit event to re-render any part depending on the connection status
                            self.state_sender.send(state.clone())?;

                            match create_server_handle("termplay.xyz").await {
                                Ok(server_handle) => {
                                    // set the server handle and change status for further processing
                                    let _ = opt_server_handle.insert(server_handle);
                                    state.connection_status = ConnectionStatus::Connected;
                                    // ticker needs to be resetted to avoid showing time spent inputting and connecting to the server address
                                    // ticker.reset();
                                },
                                Err(err) => {
                                    state.connection_status = ConnectionStatus::Errored {message: err.to_string()};
                                }
                            }
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
