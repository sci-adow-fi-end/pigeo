use simple_logger::SimpleLogger;
use std::io;
mod client;
mod comunication;
mod server;

fn main() {
    SimpleLogger::new().init().unwrap();

    let mut input = String::new();

    println!("press 1 to be a client, 2 to be a server");

    io::stdin()
        .read_line(&mut input) // Read a line into the mutable String
        .expect("Failed to read line"); // Handle potential errors

    if input.trim() == "2" {
        let mut server = match server::server::Server::init_connection() {
            None => {
                panic!();
            }
            Some(res) => res,
        };

        server.listen();
    } else if input.trim() == "1" {
        let mut client = client::client::Client::new();
    }
}

#[cfg(test)]
mod tests {

    use core::panic;
    use log::{info, warn};
    use openssl::ssl::{SslAcceptor, SslConnector, SslFiletype, SslMethod};
    use simple_logger::SimpleLogger;
    use std::io;
    use std::net::TcpListener;
    use std::net::TcpStream;
    use std::sync::mpsc;
    use std::sync::Arc;
    use std::thread;

    use crate::client::client::Client;
    use crate::server;
    use crate::server::server::Server;

    #[test]
    fn test_register() {
        //init of the logger
        SimpleLogger::new().init().unwrap();

        //init of the sinchronization channel
        let (tx, rx) = mpsc::channel();

        //__________________________________________________________________________________CLIENT SIDE
        let client_thread = thread::spawn(move || {
            info!("the client spawned");

            let _response: bool = rx.recv().unwrap();
            let mut client = Client::new();
            assert!(client
                .register("pippo".to_string(), "baudo".to_string())
                .is_ok())
        });

        //__________________________________________________________________________________SERVER SIDE
        let server_thread = thread::spawn(move || {
            info!("the server spawned");

            let ready: bool = true;
            let mut server = match Server::init_connection() {
                Some(server) => server,
                None => panic!(),
            };
            server.listen();

            tx.send(ready).unwrap();
        });

        assert!(client_thread.join().is_ok());
    }
}

/*
fn main() {
    //init of the logger
    SimpleLogger::new().init().unwrap();

    //init of the sinchronization channel
    let (tx, rx) = mpsc::channel();

    //__________________________________________________________________________________CLIENT SIDE
    let client = thread::spawn(move || {
        info!("the client spawned");

        let _response: bool = rx.recv().unwrap();
        let mut connector_builder = SslConnector::builder(SslMethod::tls()).unwrap();

        connector_builder.set_ca_file("certs.pem").unwrap();
        info!("you are using a self signed certificate, only for testing pourposes");

        let connector = connector_builder.build();

        let stream = match TcpStream::connect("localhost:3000") {
            Ok(res) => res,
            Err(_e) => {
                warn!("the client cannot connect to the server");
                panic!();
            }
        };

        let mut stream = match connector.connect("localhost", stream) {
            Ok(res) => res,
            Err(_e) => {
                warn!("the client cant have a secure connection, aborting");
                panic!();
            }
        };

        let message = b"yabadabadooo!";
        stream.write_all(message).unwrap();

        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).unwrap();

        println!(
            "Risposta dal server: {}",
            buffer
                .iter()
                .take(bytes_read)
                .map(|&n| n as char)
                .collect::<String>()
        );

        println!("ho finitoo");
    });

    //__________________________________________________________________________________SERVER SIDE
    let server = thread::spawn(move || {
        info!("the server spawned");

        let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

        match acceptor.set_private_key_file("key.pem", SslFiletype::PEM) {
            Ok(_res) => {}
            Err(_e) => {
                warn!("key file not found");
                panic!();
            }
        }

        match acceptor.set_certificate_chain_file("certs.pem") {
            Ok(_res) => {}
            Err(_e) => {
                warn!("certificate file not found");
                panic!();
            }
        }

        match acceptor.check_private_key() {
            Ok(_res) => {}
            Err(_e) => {
                warn!("private key not valid");
                panic!();
            }
        }

        let acceptor = Arc::new(acceptor.build());

        let listener = match TcpListener::bind("localhost:3000") {
            Ok(res) => res,
            Err(_e) => {
                warn!("the server cannot bind correctly");
                panic!();
            }
        };

        let ready: bool = true;
        tx.send(ready).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let acceptor = acceptor.clone();

                    thread::spawn(move || {
                        let mut stream = acceptor.accept(stream).unwrap();
                        let mut buffer = [0; 1024];
                        let bytes_read = stream.read(&mut buffer).unwrap();

                        println!(
                            "Ricevuti {} byte: {}",
                            bytes_read,
                            buffer
                                .iter()
                                .take(bytes_read)
                                .map(|&n| n as char)
                                .collect::<String>()
                        );

                        let response = b"message saved";
                        stream.write_all(response).unwrap();

                        println!(
                            "Risposta inviata: {}",
                            buffer
                                .iter()
                                .take(bytes_read)
                                .map(|&n| n as char)
                                .collect::<String>()
                        );
                    });
                }
                Err(_e) => {
                    warn!("error accepting the connection");
                }
            }
        }
    });

    //__________________________________________________________________________________OTHER STUFF
    server.join().unwrap();
    client.join().unwrap();
}
*/
