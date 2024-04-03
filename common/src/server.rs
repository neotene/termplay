use tokio::net::TcpStream;
use tokio::io::{ AsyncBufReadExt, AsyncWriteExt, BufStream };
use tokio_native_tls::TlsStream;

use crate::command;

pub type ClientStream = BufStream<TlsStream<TcpStream>>;

pub struct CommandManager {
    buf_stream: ClientStream,
}

pub const NEW_LINE: &[u8; 2] = b"\r\n";

impl CommandManager {
    pub fn new(buf_stream: ClientStream) -> Self {
        Self { buf_stream }
    }

    /// Send a [crate::command::UserCommand] to the backing [TcpStream]
    ///
    /// # Cancel Safety
    ///
    /// This method is not cancellation safe. If it is used as the event
    /// in a [tokio::select!] statement and some other
    /// branch completes first, then the provided [crate::command::UserCommand] may have been
    /// partially written, but future calls to `write` will start over
    /// from the beginning of the buffer. Causing undefined behaviour.
    pub async fn send(&mut self, command: &command::ServerCommand) -> anyhow::Result<()> {
        let mut serialized_bytes = serde_json::to_vec(command)?;
        serialized_bytes.extend_from_slice(NEW_LINE);
        self.buf_stream.write(&serialized_bytes).await?;
        self.buf_stream.flush().await?;
        Ok(())
    }

    pub async fn receive<T>(&mut self) -> anyhow::Result<command::UserCommand> {
        let mut buffer: Vec<u8> = Vec::new();
        self.buf_stream.read_until(b'\n', &mut buffer).await?;
        let str = std::str::from_utf8(&buffer)?;
        println!("Received: {}", str);
        match serde_json::from_slice(&buffer) {
            Ok(cmd) => Ok(cmd),
            Err(e) => Err(anyhow::anyhow!("Error deserializing command: {}", e)),
        }
    }
}
