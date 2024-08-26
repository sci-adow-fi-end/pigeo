use log::{info, warn};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

struct Server {
    connection_listener: TcpListener,
    conncetion_acceptor: Arc<SslAcceptor>,
}
impl Server {
    fn new() -> Self {
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
        Server {
            connection_listener: listener,
            conncetion_acceptor: acceptor,
        }
    }

    fn listen(self) {
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
