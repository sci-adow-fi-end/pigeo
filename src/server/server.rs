use crate::server::dao::DAO;
use log::{info, warn};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;
pub struct Server {
    server_dao: DAO,
    connection_listener: TcpListener,
    conncetion_acceptor: Arc<SslAcceptor>,
}
impl Server {
    pub fn init_connection() -> Option<Self> {
        let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

        match acceptor.set_private_key_file("key.pem", SslFiletype::PEM) {
            Ok(_res) => {}
            Err(_e) => {
                warn!("key file not found");
                return None;
            }
        };

        match acceptor.set_certificate_chain_file("certs.pem") {
            Ok(_res) => {}
            Err(_e) => {
                warn!("certificate file not found");
                return None;
            }
        };

        match acceptor.check_private_key() {
            Ok(_res) => {}
            Err(_e) => {
                warn!("private key not valid");
                return None;
            }
        };

        let acceptor = Arc::new(acceptor.build());

        let listener = match TcpListener::bind("localhost:3000") {
            Ok(res) => res,
            Err(_e) => {
                warn!("the server cannot bind correctly");
                return None;
            }
        };

        let dao = match DAO::init_connection() {
            Some(res) => res,
            None => {
                warn!("the database cannot be connected");
                return None;
            }
        };

        Some(Server {
            server_dao: dao,
            connection_listener: listener,
            conncetion_acceptor: acceptor,
        })
    }

    pub fn listen(self) {
        for stream in self.connection_listener.incoming() {
            match stream {
                Ok(stream) => {
                    let acceptor = self.conncetion_acceptor.clone();

                    thread::spawn(move || {
                        let mut stream = match acceptor.accept(stream) {
                            Ok(res) => res,
                            Err(_e) => {
                                warn!("the request could not be accepted");
                                panic!();
                            }
                        }; //FIXME add more info too the error

                        let mut buffer = [0; 2048];
                        let bytes_read = match stream.read(&mut buffer) {
                            Ok(res) => res,
                            Err(_e) => {
                                warn!("message read failed");
                                panic!();
                            }
                        };

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
    }
}
