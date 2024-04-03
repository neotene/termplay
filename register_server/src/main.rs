use common::server::CommandManager;
use common::command::{ RegisterCommand, RegisterResponseCommand, ServerCommand, UserCommand };

use tokio::net::{ TcpListener, TcpStream };
use tokio_native_tls::{ native_tls, TlsAcceptor };
use std::env;
use std::fs::File;
use std::io::Read;

use native_tls::Identity;

use lettre::{ Message, SmtpTransport, Transport };
use lettre::transport::smtp::authentication::Credentials;
use std::error::Error;

// Fonction pour envoyer un e-mail
fn _send_email(
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

fn _send_email_configured(
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

    let conf_file_path = env::args().nth(1).unwrap();
    let conf = ini!(conf_file_path.as_str());

    let cert_file_path = conf["ssl"]["cert file path"].clone().unwrap();
    let key_file_path = conf["ssl"]["key file path"].clone().unwrap();

    println!("Loading cerfiticate and private key files");
    let mut cert_file = File::open(cert_file_path)?;
    let mut key_file = File::open(key_file_path)?;

    let mut key_buffer = Vec::new();
    let mut cert_buffer = Vec::new();
    match cert_file.read_to_end(&mut cert_buffer) {
        Ok(_) => println!("Certificat lu avec succès"),
        Err(e) => eprintln!("Erreur lors de la lecture du certificat : {}", e),
    }
    match key_file.read_to_end(&mut key_buffer) {
        Ok(_) => println!("Clé privée lue avec succès"),
        Err(e) => eprintln!("Erreur lors de la lecture de la clé privée : {}", e),
    }

    println!("Configuring TLS acceptor with certificate and private key");
    let acceptor = {
        let identity = Identity::from_pkcs12(&cert_buffer, "")?;
        let builder = native_tls::TlsAcceptor::new(identity)?;
        TlsAcceptor::from(builder)
    };

    println!("Starting server...");
    let addr = env::args().nth(2).unwrap();
    let listener = TcpListener::bind(&addr).await?;
    println!("Server started on {}", addr);

    println!("Listening for incoming connections");
    loop {
        let (socket, _) = listener.accept().await?;
        let acceptor = acceptor.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, acceptor).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}

async fn handle_connection(socket: TcpStream, acceptor: TlsAcceptor) -> anyhow::Result<()> {
    println!("Accepting TLS connection...");
    let tls_stream = acceptor.accept(socket).await.ok().unwrap();

    println!("TLS connection accepted");

    let stream = tokio::io::BufStream::new(tls_stream);
    let mut cmd_manager = CommandManager::new(stream);

    println!("Waiting for command...");
    match cmd_manager.receive::<UserCommand>().await {
        Ok(cmd) => {
            println!("Commande reçue : {:?}", cmd);
            match cmd {
                UserCommand::Register(RegisterCommand { login, password }) => {
                    println!(
                        "Enregistrement de l'utilisateur {} avec le mot de passe {}",
                        login,
                        password
                    );
                    cmd_manager.send(
                        &ServerCommand::RegisterResponse(RegisterResponseCommand {
                            email_sent: true,
                        })
                    ).await?;
                }
            }

            Ok(())
        }
        Err(e) => {
            eprintln!();
            Err(anyhow::anyhow!("Error receiving command: {}", e))
        }
    }
}
