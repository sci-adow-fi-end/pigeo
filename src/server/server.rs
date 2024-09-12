use crate::comunication::message_type::{Answer, Request};
use crate::server::dao::DAO;
use log::{info, warn};
use openssl::aes::wrap_key;
use openssl::error::Error;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslStream};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
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

    pub fn handle_register(
        &mut self,
        username: String,
        password: String,
        public_key: String,
    ) -> Answer {
        let istaken = match self.server_dao.is_name_present(&username) {
            Err(e) => return e.into_answer(),
            Ok(val) => val,
        };

        if istaken {
            return Answer::BadName;
        }

        match self.server_dao.save_user(&username, &password, &public_key) {
            Err(e) => return e.into_answer(),
            Ok(()) => return Answer::Ok,
        }
    }

    pub fn handle_send(
        &mut self,
        username: &String,
        password: &String,
        receiver: &String,
        message: &String,
    ) -> Answer {
        let valid_name = match self.server_dao.is_name_present(&username) {
            Err(e) => return e.into_answer(),
            Ok(val) => val,
        };

        let valid_password = match self.server_dao.validate_credentials(&username, &password) {
            Err(e) => return e.into_answer(),
            Ok(val) => val,
        };

        if !valid_name {
            return Answer::BadName;
        }

        if !valid_password {
            return Answer::BadPwd;
        }

        //TODO encrypt message

        match self.server_dao.save_message(&message, &username, &receiver) {
            Err(e) => return e.into_answer(),
            Ok(()) => return Answer::Ok,
        }
    }

    pub fn handle_receive(
        &mut self,
        username: &String,
        password: &String,
        sender: &String,
    ) -> Answer {
        let valid_name = match self.server_dao.is_name_present(&username) {
            Err(e) => return e.into_answer(),
            Ok(val) => val,
        };

        let valid_password = match self.server_dao.validate_credentials(&username, &password) {
            Err(e) => return e.into_answer(),
            Ok(val) => val,
        };

        if !valid_name {
            return Answer::BadName;
        }

        if !valid_password {
            return Answer::BadPwd;
        }

        //TODO encrypt message

        match self
            .server_dao
            .get_messages_by_sender_receiver(sender, username)
        {
            Err(e) => return e.into_answer(),
            Ok(messages) => return Answer::Messages(messages),
        }
    }

    pub fn get_request(
        stream: SslStream<TcpStream>,
    ) -> Result<Request, Box<dyn std::error::Error>> {
        let mut buffer = [0; 2048];
        let bytes_read = match stream.read(&mut buffer) {
            Ok(res) => res,
            Err(e) => {
                warn!("message read failed");
                return Err(Box::new(e));
            }
        };

        let request_string: String = String::from_utf8_lossy(&buffer[..bytes_read]);

        return match Request::decode(&request_string) {
            Ok(res) => Ok(res),
            Err(e) => {
                warn!("message decode failed");
                Err(Box::new(e))
            }
        };
    }

    pub fn examine_request(&mut self, client_request: Request) -> Answer {
        match client_request {
            Request::Register {
                username,
                password,
                public_key,
            } => {
                return self.handle_register(username, password, public_key);
            }
            Request::Send {
                username,
                password,
                message,
                receiver,
            } => {
                return self.handle_send(&username, &password, &receiver, &message);
            }
            Request::Receive {
                username,
                password,
                sender,
            } => {
                return self.handle_receive(&username, &password, &sender);
            }
        }
    }

    pub fn send_answer(
        stream: &mut SslStream<TcpStream>,
        answer: Answer,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response = match answer.encode() {
            Err(e) => {
                warn!("failed to encode answer");
                return Err(Box::new(e));
            }
            Ok(res) => res,
        };

        return match stream.write_all(response.as_bytes()) {
            Err(e) => {
                warn!("failed to encode answer");
                Err(Box::new(e))
            }
            Ok(res) => Ok(res),
        };
    }

    pub fn listen(&mut self) {
        for stream in self.connection_listener.incoming() {
            let acceptor = self.conncetion_acceptor.clone();

            thread::spawn(move || {
                let stream = match stream {
                    Err(_e) => {
                        warn!("error accepting the connection");
                        panic!();
                    }
                    Ok(stream) => stream,
                };

                let mut accepted_stream = match acceptor.accept(stream) {
                    Ok(res) => res,
                    Err(_e) => {
                        warn!("the request could not be accepted");
                        panic!();
                    }
                };

                let request = Self::get_request(&mut accepted_stream).unwrap();
                let answer = self.examine_request(request);
                Self::send_answer(&mut accepted_stream, answer).unwrap();
            });
        }
    }
}

/*println!(
    "Ricevuti {} byte: {}",
    bytes_read,
    buffer
        .iter()
        .take(bytes_read)
        .map(|&n| n as char)
        .collect::<String>()
);*/
