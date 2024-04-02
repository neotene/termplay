use tokio::net::TcpStream;
use tokio::io::{ AsyncWriteExt, AsyncReadExt };
use tokio_native_tls::{ native_tls, TlsConnector };
use native_tls::{ Certificate, TlsConnector as NativeTlsConnector };
use std::env;
use std::fs::File;
use std::io::Read;
use std::net::ToSocketAddrs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Charger le certificat du serveur (facultatif, pour la vérification du certificat)
    println!("Chargement du certificat du serveur");
    let mut cert_file = File::open("../cert.pem")?;
    let mut cert_buffer = Vec::new();
    cert_file.read_to_end(&mut cert_buffer)?;
    let cert = Certificate::from_pem(&cert_buffer)?;

    // Configurer le connecteur TLS avec le certificat du serveur (facultatif)
    println!("Configuration du connecteur TLS avec le certificat du serveur");
    let mut tls_connector = NativeTlsConnector::builder();
    tls_connector.add_root_certificate(cert);
    tls_connector.danger_accept_invalid_certs(true);

    let args: Vec<String> = env::args().collect();

    // Vérifiez que l'adresse du serveur est spécifiée
    if args.len() < 3 {
        eprintln!("Usage: client_cli host port");
        std::process::exit(1);
    }

    let host = &args[1];
    let port = &args[2];

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

    // Connectez-vous au serveur avec SSL
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

    // Envoie le nom d'utilisateur et le mot de passe au serveur
    println!("Envoi du nom d'utilisateur et du mot de passe au serveur");
    let mut stream = tokio::io::BufStream::new(stream);
    let username = "utilisateur";
    let password = "motdepasse";
    stream.write_all(format!("{} {}\n", username, password).as_bytes()).await?;

    // Attend une réponse du serveur
    println!("Attente d'une réponse du serveur");
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer).await?;
    println!("Réponse du serveur : {}", buffer);

    Ok(())
}
