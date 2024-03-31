use tokio::net::{ TcpListener, TcpStream };
use tokio_native_tls::{ native_tls, TlsAcceptor };
use std::borrow::BorrowMut;
use std::fs::File;
use std::io::{ self, BufReader, Read };
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Charger le certificat et la clé privée TLS
    let cert_file = File::open("server.crt")?;
    let key_file = File::open("server.key")?;
    let mut cert_buffer = Vec::new();
    let mut key_buffer = Vec::new();
    cert_file.read_to_end(&mut cert_buffer)?;
    key_file.read_to_end(&mut key_buffer)?;

    // Convertir les vecteurs d'octets en chaînes de caractères
    let cert_string = std::str::from_utf8(&cert_buffer)?;
    let key_string = std::str::from_utf8(&key_buffer)?;

    // Convert the strings to byte slices
    let cert_bytes = cert_string.as_bytes();
    let key_bytes = key_string.as_bytes();

    // Create a TLS acceptor
    let acceptor = {
        let identity = native_tls::Identity::from_pkcs12(cert_bytes, key_string)?;
        let mut config = native_tls::TlsAcceptor::builder(identity).borrow_mut();
        // Configure ALPN protocols
        let mut config = rustls::ServerConfig::new(rustls::NoClientAuth::new());
        config.set_protocols(&[b"http/1.1".to_vec()]);

        builder.set_rustls_server_config(config);
        TlsAcceptor::from(Arc::new(builder.build()?))
    };

    // Create a TCP listener
    let addr = "127.0.0.1:8080".parse()?;
    let listener = TcpListener::bind(&addr).await?;
    println!("Server started and listening on port 8080...");

    loop {
        // Accepter les connexions entrantes
        let (socket, _) = listener.accept().await?;
        let acceptor = acceptor.clone();

        // Gérer chaque connexion dans un thread séparé
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, acceptor).await {
                eprintln!("Erreur lors de la gestion de la connexion : {}", e);
            }
        });
    }
}

async fn handle_connection(socket: TcpStream, acceptor: TlsAcceptor) -> Result<(), io::Error> {
    // Accepter la connexion TLS
    let tls_stream = acceptor.accept(socket).await?;

    // Manipuler les données TLS
    // Ici vous pouvez gérer les données chiffrées reçues sur le flux TLS

    Ok(())
}
