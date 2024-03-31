use tokio::net::TcpStream;

use tokio_native_tls::native_tls::TlsConnector;
use std::net::ToSocketAddrs;
use std::pin::Pin;

pub type SSLStream = tokio_native_tls::TlsStream<TcpStream>;

type ServerHandle = Pin<Box<SSLStream>>;

// pub type BoxedStream<Item> = Pin<Box<dyn SSLStream<Item = Item> + Send>>;

// pub type EventStream = BoxedStream<anyhow::Result<event::Event>>;

// impl CommandWriter {
//     pub fn new(writer: SSLStream) -> Self {
//         Self { socket: writer }
//     }

//     pub async fn write(&mut self, command: &command::UserCommand) -> anyhow::Result<()> {
//         let mut serialized_bytes = serde_json::to_vec(command)?;
//         serialized_bytes.extend_from_slice(NEW_LINE);

//         self.socket.write_all(serialized_bytes.as_slice()).await?;

//         Ok(())
//     }
// }

pub async fn create_server_handle() -> anyhow::Result<ServerHandle> {
    let addr = "termplay.xyz:443".to_socket_addrs()?.next().unwrap();

    let socket = TcpStream::connect(&addr).await?;
    let cx = TlsConnector::builder().build()?;
    let cx = tokio_native_tls::TlsConnector::from(cx);

    let socket = cx.connect("termplay.xyz", socket).await?;

    Ok(Pin::new(Box::new(socket)))
}
