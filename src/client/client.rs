use crate::comunication::message_type::Answer;
use crate::comunication::message_type::Request;
use log::{info, warn};
use openssl::rsa::Rsa;
use openssl::ssl::{SslConnector, SslMethod, SslStream};
use std::error::Error;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Read;
use std::io::{self, Write};
use std::net::TcpStream;
use std::path::Path;

enum RegistrationError {
    BadUsername,
    ServerError,
    Unknown,
}

enum SendError {
    BadUsername,
    BadPassword,
    BadReceiver,
    ServerError,
    Unknown,
}

enum ReceiveError {
    BadUsername,
    BadPassword,
    BadSender,
    ServerError,
    Unknown,
}

struct Client {
    connector: SslConnector,
    stream: SslStream<TcpStream>,
    private_key: String,
    public_key: String,
}

impl Client {
    fn new() -> Self {
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

        let private_key_path = Path::new("private_key.pem");
        let public_key_path = Path::new("public_key.pem");

        let rsa = match Rsa::generate(2048) {
            Ok(res) => res,
            Err(_e) => {
                warn!("the rsa keys could not be created");
                panic!();
            }
        };

        if private_key_path.exists() || public_key_path.exists() {
            let private_key_pem = rsa.private_key_to_pem().unwrap();
            let mut private_key_file = File::create(&private_key_path).unwrap();
            private_key_file.write_all(&private_key_pem).unwrap();

            let public_key_pem = rsa.public_key_to_pem().unwrap();
            let mut public_key_file = File::create(&public_key_path).unwrap();
            public_key_file.write_all(&public_key_pem).unwrap();
        }

        let private_key = read_to_string(private_key_path).unwrap();
        let public_key = read_to_string(public_key_path).unwrap();

        Client {
            connector,
            stream,
            private_key,
            public_key,
        }
    }

    fn register(&mut self, username: String, password: String) -> Result<(), RegistrationError> {
        let req = Request::Register {
            username,
            password,
            public_key: self.private_key.clone(),
        };

        let message: Option<String>;

        match req.encode() {
            Err(_e) => {
                warn!("failed to encode the message");
                return Err(RegistrationError::Unknown);
            }
            Ok(res) => {
                message = Some(res);
            }
        };

        match message {
            None => {
                warn!("message format not valid");
                Err(RegistrationError::Unknown)
            }
            Some(message) => {
                match self.stream.write_all(message.as_bytes()) {
                    Ok(_res) => {}
                    Err(_e) => {
                        warn!("failed to write the message");
                        return Err(RegistrationError::Unknown);
                    }
                };

                let mut buffer = [0; 2048];

                let bytes_read: Option<usize>;

                match self.stream.read(&mut buffer) {
                    Ok(res) => {
                        bytes_read = Some(res);
                    }
                    Err(_e) => {
                        warn!("failed to read the response");
                        return Err(RegistrationError::Unknown);
                    }
                }

                match bytes_read {
                    None => {
                        warn!("response not valid");
                        Err(RegistrationError::Unknown)
                    }
                    Some(bytes_read) => {
                        let response = String::from_utf8_lossy(&buffer[..bytes_read]);

                        return match Answer::decode(&response) {
                            Err(_e) => {
                                warn!("failed to decode the response");
                                Err(RegistrationError::Unknown)
                            }
                            Ok(answ) => match answ {
                                Answer::Ok => Ok(()),
                                Answer::BadName => {
                                    warn!("the username entered already exists");
                                    Err(RegistrationError::BadUsername)
                                }
                                Answer::BadSender | Answer::BadPwd | Answer::BadReceiver => {
                                    Err(RegistrationError::Unknown)
                                }

                                Answer::Messages(_mess) => Err(RegistrationError::Unknown),

                                Answer::ServerError => Err(RegistrationError::ServerError),
                            },
                        };
                    }
                }
            }
        }
    }

    fn send(
        &mut self,
        username: String,
        password: String,
        message: String,
        receiver: String,
    ) -> Result<(), SendError> {
        let req = Request::Send {
            username,
            password,
            message,
            receiver,
        };

        let message: Option<String>;

        match req.encode() {
            Err(_e) => {
                warn!("failed to encode the message");
                return Err(SendError::Unknown);
            }
            Ok(res) => {
                message = Some(res);
            }
        };

        match message {
            None => {
                warn!("message format not valid");
                Err(SendError::Unknown)
            }
            Some(message) => {
                match self.stream.write_all(message.as_bytes()) {
                    Ok(_res) => {}
                    Err(_e) => {
                        warn!("failed to write the message");
                        return Err(SendError::Unknown);
                    }
                };

                let mut buffer = [0; 2048];

                let bytes_read: Option<usize>;

                match self.stream.read(&mut buffer) {
                    Ok(res) => {
                        bytes_read = Some(res);
                    }
                    Err(_e) => {
                        warn!("failed to read the response");
                        return Err(SendError::Unknown);
                    }
                }

                match bytes_read {
                    None => {
                        warn!("response not valid");
                        Err(SendError::Unknown)
                    }
                    Some(bytes_read) => {
                        let response = String::from_utf8_lossy(&buffer[..bytes_read]);

                        return match Answer::decode(&response) {
                            Err(_e) => {
                                warn!("failed to decode the response");
                                Err(SendError::Unknown)
                            }
                            Ok(answ) => match answ {
                                Answer::Ok => Ok(()),
                                Answer::BadName => {
                                    warn!("the username entered doesen't exist");
                                    Err(SendError::BadUsername)
                                }
                                Answer::BadPwd => {
                                    warn!("the password entered is not correct");
                                    Err(SendError::BadUsername)
                                }
                                Answer::BadReceiver => {
                                    warn!("the reciever specified does not exist");
                                    Err(SendError::BadUsername)
                                }

                                Answer::BadSender => Err(SendError::Unknown),

                                Answer::Messages(_mess) => Err(SendError::Unknown),

                                Answer::ServerError => Err(SendError::ServerError),
                            },
                        };
                    }
                }
            }
        }
    }

    fn receive(
        &mut self,
        username: String,
        password: String,
        sender: String,
    ) -> Result<Vec<String>, ReceiveError> {
        let req = Request::Receive {
            username,
            password,
            sender,
        };

        let message: Option<String>;

        match req.encode() {
            Err(_e) => {
                warn!("failed to encode the message");
                return Err(ReceiveError::Unknown);
            }
            Ok(res) => {
                message = Some(res);
            }
        };

        match message {
            None => {
                warn!("message format not valid");
                Err(ReceiveError::Unknown)
            }
            Some(message) => {
                match self.stream.write_all(message.as_bytes()) {
                    Ok(_res) => {}
                    Err(_e) => {
                        warn!("failed to write the message");
                        return Err(ReceiveError::Unknown);
                    }
                };

                let mut buffer = [0; 2048];

                let bytes_read: Option<usize>;

                match self.stream.read(&mut buffer) {
                    Ok(res) => {
                        bytes_read = Some(res);
                    }
                    Err(_e) => {
                        warn!("failed to read the response");
                        return Err(ReceiveError::Unknown);
                    }
                }

                match bytes_read {
                    None => {
                        warn!("response not valid");
                        Err(ReceiveError::Unknown)
                    }
                    Some(bytes_read) => {
                        let response = String::from_utf8_lossy(&buffer[..bytes_read]);

                        return match Answer::decode(&response) {
                            Err(_e) => {
                                warn!("failed to decode the response");
                                Err(ReceiveError::Unknown)
                            }
                            Ok(answ) => match answ {
                                Answer::Ok => Err(ReceiveError::Unknown),
                                Answer::BadName => {
                                    warn!("the username entered doesen't exist");
                                    Err(ReceiveError::BadUsername)
                                }

                                Answer::BadPwd => {
                                    warn!("the password entered is not correct");
                                    Err(ReceiveError::BadUsername)
                                }
                                Answer::BadSender => {
                                    warn!("the sender specified doesen't exist");
                                    Err(ReceiveError::BadUsername)
                                }

                                Answer::BadReceiver => Err(ReceiveError::Unknown),

                                Answer::Messages(mess) => Ok(mess),

                                Answer::ServerError => Err(ReceiveError::ServerError),
                            },
                        };
                    }
                }
            }
        }
    }
}
