use ini::configparser::ini::Ini;
use tokio::io::AsyncReadExt;
use tokio::net::{ TcpListener, TcpStream };
use tokio_native_tls::{ native_tls, TlsAcceptor };
use std::env;
use std::fs::File;
use std::io::{ self, Read };

use native_tls::Identity;

use lettre::{ Message, SmtpTransport, Transport };
use lettre::transport::smtp::authentication::Credentials;
use std::error::Error;

// Fonction pour envoyer un e-mail
fn send_email(
    smtp_server: String,
    smtp_port: u16,
    username: String,
    password: String,
    sender: String,
    recipient: String,
    subject: String,
    body: String
) -> Result<(), Box<dyn Error>> {
    // Créer les informations d'authentification SMTP
    let credentials = Credentials::new(username.to_string(), password.to_string());

    // Créer le transport SMTP
    let mailer = SmtpTransport::relay(smtp_server.as_str())?
        .credentials(credentials)
        .port(smtp_port)
        .build();

    // Créer le message e-mail
    let email = Message::builder()
        .from(sender.parse()?)
        .to(recipient.parse()?)
        .subject(subject)
        .body(body)?;

    // Envoyer l'e-mail
    mailer.send(&email)?;

    Ok(())
}

fn send_email_configured(
    conf_path: &str,
    sender: String,
    recipient: String,
    subject: String,
    body: String
) -> Result<(), Box<dyn Error>> {
    // Load the INI file
    let conf = ini!(conf_path);

    // Read the SMTP server configuration from the INI file
    let smtp_server = conf["smtp"]["server"].clone().unwrap();
    let smtp_port = conf["smtp"]["port"].clone().unwrap().parse::<u16>()?;
    let username = conf["smtp"]["username"].clone().unwrap();
    let password = conf["smtp"]["password"].clone().unwrap();

    // Create the email message
    let email = Message::builder()
        .from(sender.parse()?)
        .to(recipient.parse()?)
        .subject(subject)
        .body(body)?;

    // Create the SMTP transport with the configured server and credentials
    let mailer = SmtpTransport::relay(smtp_server.as_str())?
        .credentials(Credentials::new(username, password))
        .port(smtp_port)
        .build();

    // Send the email
    mailer.send(&email)?;

    Ok(())
}

#[macro_use]
extern crate ini;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if env::args().len() < 3 {
        eprintln!("Usage: {} <config file path> <bind host>", env::args().next().unwrap());
        std::process::exit(1);
    }

    // Read the INI file
    let conf_file_path = env::args().nth(1).unwrap();
    let conf = ini!(conf_file_path.as_str());

    let cert_file_path = conf["ssl"]["cert file path"].clone().unwrap();
    let key_file_path = conf["ssl"]["key file path"].clone().unwrap();
    // Read the SMTP server configuration from the INI file
    // Charger le certificat et la clé privée TLS
    println!("Chargement du certificat");
    let mut cert_file = File::open(cert_file_path)?;

    println!("Chargement de la clé privée");
    let mut key_file = File::open(key_file_path)?;

    let mut cert_buffer = Vec::new();
    let mut key_buffer = Vec::new();

    println!("Lecture du certificat et de la clé privée");
    match cert_file.read_to_end(&mut cert_buffer) {
        Ok(_) => println!("Certificat lu avec succès"),
        Err(e) => eprintln!("Erreur lors de la lecture du certificat : {}", e),
    }
    match key_file.read_to_end(&mut key_buffer) {
        Ok(_) => println!("Clé privée lue avec succès"),
        Err(e) => eprintln!("Erreur lors de la lecture de la clé privée : {}", e),
    }

    // Créer un accepteur TLS
    println!("Création de l'accepteur TLS");
    let acceptor = {
        let identity = Identity::from_pkcs12(&cert_buffer, "")?;
        let builder = native_tls::TlsAcceptor::new(identity)?;
        TlsAcceptor::from(builder)
    };

    println!("Démarrage du serveur");
    // Créer un écouteur TCP
    let addr = env::args().nth(2).unwrap();
    let listener = TcpListener::bind(&addr).await?;
    println!("Serveur démarré et en écoute sur le port 8080...");

    println!("En attente de connexions entrantes");
    loop {
        // Accepter les connexions entrantes
        let (socket, _) = listener.accept().await?;
        let acceptor = acceptor.clone();

        // Gérer chaque connexion dans un thread séparé
        // tokio::spawn(async move {
        if let Err(e) = handle_connection(socket, acceptor).await {
            eprintln!("Erreur lors de la gestion de la connexion : {}", e);
        }
        // });
    }
}

async fn handle_connection(socket: TcpStream, acceptor: TlsAcceptor) -> Result<(), io::Error> {
    println!("Connexion entrante reçue");
    // Accepter la connexion TLS
    let mut tls_stream = acceptor.accept(socket).await.ok().unwrap();

    // Lire les données TLS
    println!("Lecture des données TLS");
    let mut buffer = [0; 1024];
    let resp = tls_stream.read(&mut buffer).await?;
    println!("Données reçues : {:?}", &buffer[..resp]);

    // Manipuler les données TLS
    // Ici vous pouvez gérer les données chiffrées reçues sur le flux TLS

    Ok(())
}
