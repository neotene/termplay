use tokio::io::AsyncReadExt;
use tokio::net::{ TcpListener, TcpStream };
use tokio_native_tls::{ native_tls, TlsAcceptor };
use std::fs::File;
use std::io::{ self, Read };

use native_tls::Identity;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Charger le certificat et la clé privée TLS
    println!("Chargement du certificat");
    let mut cert_file = File::open("../keyStore.p12")?;

    println!("Chargement de la clé privée");
    let mut key_file = File::open("../myKey.pem")?;

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
        Identity::from;
        let identity = Identity::from_pkcs12(&cert_buffer, "")?;
        let builder = native_tls::TlsAcceptor::new(identity)?;
        TlsAcceptor::from(builder)
    };

    println!("Démarrage du serveur");
    // Créer un écouteur TCP
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr).await?;
    println!("Serveur démarré et en écoute sur le port 8080...");

    println!("En attente de connexions entrantes");
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
    println!("Connexion entrante reçue");
    // Accepter la connexion TLS
    let mut tls_stream = acceptor.accept(socket).await.ok().unwrap();

    // Lire les données TLS
    let mut buffer = [0; 1024];
    let resp = tls_stream.read(&mut buffer).await?;
    println!("Données reçues : {:?}", &buffer[..resp]);

    // Manipuler les données TLS
    // Ici vous pouvez gérer les données chiffrées reçues sur le flux TLS

    Ok(())
}
