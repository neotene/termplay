use common::client::CommandManager;
use common::command::{ self, RegisterCommand, RegisterResponseCommand };
use tokio::net::TcpStream;
use tokio_native_tls::{ native_tls, TlsConnector };
use native_tls::{ Certificate, TlsConnector as NativeTlsConnector };
use std::env;
use std::fs::File;
use std::io::Read;
use std::net::ToSocketAddrs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if env::args().len() < 6 {
        eprintln!("Usage: client_cli host port cert_file_path username password");
        std::process::exit(1);
    }

    let host = &env::args().nth(1).unwrap();
    let port = &env::args().nth(2).unwrap();
    let cert_file_path = &env::args().nth(3).unwrap();
    let username = &env::args().nth(4).unwrap();
    let password = &env::args().nth(5).unwrap();

    println!("Chargement du certificat du serveur");
    let mut cert_file = File::open(cert_file_path)?;
    let mut cert_buffer = Vec::new();
    cert_file.read_to_end(&mut cert_buffer)?;
    let cert = Certificate::from_pem(&cert_buffer)?;

    println!("Configuration du connecteur TLS avec le certificat du serveur");
    let mut tls_connector = NativeTlsConnector::builder();
    tls_connector.add_root_certificate(cert);
    tls_connector.danger_accept_invalid_certs(true);

    let full = format!("{}:{}", host, port);

    println!("Résolution de l'adresse du serveur");
    let addrs = full.to_socket_addrs();

    let first_addr = match addrs {
        Ok(addr) => addr.collect::<Vec<_>>()[0],
        Err(e) => {
            eprintln!("Erreur lors de la résolution de l'adresse : {}", e);
            std::process::exit(1);
        }
    };

    println!("Connexion au serveur avec SSL");
    let tls_connector = TlsConnector::from(tls_connector.build()?);
    println!("Connexion au serveur {} sur le port {}", first_addr.ip(), port);
    let stream = TcpStream::connect(format!("{}:{}", first_addr.ip(), port)).await?;
    println!("Connexion établie avec succès");
    println!(
        "Connexion sécurisée au serveur {} (ip: {}) sur le port {}",
        host,
        first_addr.ip(),
        port
    );
    let stream = tls_connector.connect(host, stream).await?;
    println!("Connexion sécurisée établie avec succès");

    println!("Envoi du nom d'utilisateur et du mot de passe au serveur");
    let stream = tokio::io::BufStream::new(stream);
    let mut cmd_manager = CommandManager::new(stream);
    cmd_manager.send(
        &command::UserCommand::Register(RegisterCommand {
            login: username.clone(),
            password: password.clone(),
        })
    ).await?;

    println!("Attente d'une réponse du serveur");
    match cmd_manager.receive::<RegisterResponseCommand>().await {
        Ok(cmd) => {
            println!("Réponse du serveur : {:?}", cmd);
        }
        Err(e) => {
            eprintln!("Erreur lors de la réception de la réponse du serveur : {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
