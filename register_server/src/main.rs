use core::result::Result::Ok;
use common::server::CommandManager;
use common::command::{ RegisterCommand, RegisterResponseCommand, ServerCommand, UserCommand };

use mailgun_rs::{ EmailAddress, Mailgun, MailgunRegion, Message };
use tokio::net::{ TcpListener, TcpStream };
use tokio_native_tls::{ native_tls, TlsAcceptor };
use std::collections::HashMap;
use std::env;

use std::fs::File;
use std::io::Read;

use native_tls::Identity;

async fn send_email(
    domain: String,
    key: String,
    sender: String,
    sender_name: String,
    recipient: String,
    subject: String,
    body: String
) -> anyhow::Result<()> {
    let recipient = EmailAddress::address(recipient.as_str());
    let message = Message {
        to: vec![recipient],
        subject: String::from(subject),
        html: String::from(body),
        ..Default::default()
    };

    let client = Mailgun {
        api_key: String::from(key),
        domain: String::from(domain),
        message,
    };
    let sender = EmailAddress::name_address(sender_name.as_str(), sender.as_str());

    client.async_send(MailgunRegion::EU, &sender).await?;
    Ok(())
}

async fn send_email_configured(
    conf: HashMap<String, HashMap<String, Option<String>>>,
    recipient: String
) -> anyhow::Result<()> {
    let domain = conf["mailgun"]["domain"].clone().unwrap();
    let key = conf["mailgun"]["api key"].clone().unwrap();
    let sender = conf["mailgun"]["sender"].clone().unwrap();
    let sender_name = conf["mailgun"]["sender name"].clone().unwrap();
    let subject = conf["mailgun"]["subject"].clone().unwrap();
    let body = conf["mailgun"]["body"].clone().unwrap();

    print!("Envoi de l'email à {}...", recipient);
    send_email(domain, key, sender, sender_name, recipient, subject, body).await?;
    Ok(())
}

#[macro_use]
extern crate ini;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
    cert_file.read_to_end(&mut cert_buffer)?;
    key_file.read_to_end(&mut key_buffer)?;

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

        let conf_cloned = conf.clone();
        // tokio::spawn(async move {
        if let Err(e) = handle_connection(conf_cloned, socket, acceptor).await {
            eprintln!("Error handling connection: {}", e);
        }
        // });
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
        Ok(UserCommand::Register(RegisterCommand { login, password })) => {
            println!("Registering user {} with password {}", login, password);
            match send_email_configured(conf, login).await {
                Ok(_res) => {
                    println!("Email sent successfully");
                }
                Err(e) => {
                    eprintln!("Error sending email: {}", e);
                }
            }
            cmd_manager.send(
                &ServerCommand::RegisterResponse(RegisterResponseCommand {
                    email_sent: true,
                })
            ).await?;
        }
        _ => {
            println!("Unknown command received");
        }
    }
    Ok(())
}

// Ok(UserCommand::Register(RegisterCommand { login, password })) => {
//     println!(
//         "Enregistrement de l'utilisateur {} avec le mot de passe {}",
//         login,
//         password
//     );
//     match send_email_configured(conf, login).await {
//         _res => {
//             println!("Email envoyé avec succès");
//         }
//     }
//     cmd_manager.send(
//         &ServerCommand::RegisterResponse(RegisterResponseCommand {
//             email_sent: true,
//         })
//     ).await?;
// }
