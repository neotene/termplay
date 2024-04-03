use common::server::CommandManager;
use common::command::{ RegisterCommand, RegisterResponseCommand, ServerCommand, UserCommand };

use tokio::net::{ TcpListener, TcpStream };
use tokio_native_tls::{ native_tls, TlsAcceptor };
use std::collections::HashMap;
use std::env;

use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use native_tls::Identity;

use lettre::{ Message, SmtpTransport, Transport };
use lettre::transport::smtp::authentication::Credentials;

fn send_email(
    smtp_server: String,
    smtp_port: u16,
    username: String,
    password: String,
    sender: String,
    recipient: String,
    subject: String,
    body: String
) -> anyhow::Result<()> {
    let credentials = Credentials::new(username.to_string(), password.to_string());

    let mailer = SmtpTransport::relay(smtp_server.as_str())?
        .credentials(credentials)
        .port(smtp_port)
        .build();

    match
        Message::builder().from(sender.parse()?).to(recipient.parse()?).subject(subject).body(body)
    {
        Ok(email) => {
            match mailer.send(&email) {
                Ok(_) => println!("Email sent successfully"),
                Err(e) => {
                    return Err(anyhow::anyhow!("Error sending email: {}", e));
                }
            }
        }
        Err(e) => {
            return Err(anyhow::anyhow!("Error building email: {}", e));
        }
    }

    Ok(())
}

fn send_email_configured(
    conf: HashMap<String, HashMap<String, Option<String>>>,
    recipient: String
) -> anyhow::Result<()> {
    let smtp_server = conf["smtp"]["server"].clone().unwrap();
    let smtp_port = conf["smtp"]["port"].clone().unwrap().parse::<u16>()?;
    let username = conf["smtp"]["login"].clone().unwrap();
    let password = conf["smtp"]["password"].clone().unwrap();

    send_email(
        smtp_server,
        smtp_port,
        username,
        password,
        String::from_str("register@termplay.xyz").unwrap(),
        recipient,
        String::from_str("Confirm your email").unwrap(),
        String::from_str("Please confirm your email address by clicking on the link below").unwrap()
    )
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

        let conf = conf.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(conf, socket, acceptor).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}

async fn handle_connection(
    conf: HashMap<String, HashMap<String, Option<String>>>,
    socket: TcpStream,
    acceptor: TlsAcceptor
) -> anyhow::Result<()> {
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
                    match send_email_configured(conf, login) {
                        Ok(_) => println!("Email envoyé avec succès"),
                        Err(e) => eprintln!("Erreur lors de l'envoi de l'email : {}", e),
                    }
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
