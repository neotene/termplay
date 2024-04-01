use tokio::net::TcpStream;
use tokio::io::{ AsyncWriteExt, AsyncReadExt };
use tokio_native_tls::{ native_tls, TlsConnector };
use native_tls::{ Certificate, TlsConnector as NativeTlsConnector };
use std::fs::File;
use std::io::Read;

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

    // Connectez-vous au serveur avec SSL
    println!("Connexion au serveur avec SSL");
    let tls_connector = TlsConnector::from(tls_connector.build()?);
    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    let stream = tls_connector.connect("localhost", stream).await?;

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
