use crate::{client::client, server::invalid_request_error::InvalidRequestError};
use log::warn;
use postgres::{Client, Error, GenericClient, NoTls};
use std::io::Result;

struct dao {
    db_client: Client,
}
impl dao {
    fn init_connection() -> Option<Self> {
        let connection = Client::connect(
            "host=localhost user=postgres password=secret dbname=mydb",
            NoTls,
        );
        return match connection {
            Err(_e) => {
                warn!("connection to the database failed");
                None
            }
            Ok(mut client) => {
                match client.execute(
                    "
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username VARCHAR(255) NOT NULL UNIQUE,
            password VARCHAR(255) NOT NULL,
            pubkey VARCHAR(255) NOT NULL
        )
        ",
                    &[],
                ) {
                    Err(_e) => {
                        warn!("error creating the table users");
                        None
                    }
                    Ok(_res) => {
                        match client.execute(
                            "CREATE TABLE IF NOT EXISTS messages (
    id SERIAL PRIMARY KEY,
    message TEXT NOT NULL,
    sender VARCHAR(255) NOT NULL,
    receiver VARCHAR(255) NOT NULL,
    FOREIGN KEY (sender) REFERENCES users(username),
    FOREIGN KEY (receiver) REFERENCES users(username)
);",
                            &[],
                        ) {
                            Err(_e) => {
                                warn!("error creating the table messages");
                                None
                            }
                            Ok(_res) => Some(dao { db_client: client }),
                        }
                    }
                }
            }
        };
    }

    fn save_user(
        &self,
        username: String,
        password: String,
        pubkey: String,
    ) -> Result<(), InvalidRequestError> {
        match client.execute(
            "INSERT INTO users (username, password, pubkey) VALUES ($1, $2, $3)",
            &[&username, &password, &pubkey],
        )
            Ok(_res)=>{Ok()},
            Err(e)=>{Err}
    }

    fn validate_credentials(username: String, password: String) -> Result<(), InvalidRequestError> {
        todo!()
    }

    fn save_message(
        &self,
        username: String,
        password: String,
        message: String,
        receiver: String,
    ) -> Result<(), InvalidRequestError> {
        todo!();
    }

    fn get_messages_by_receiver(
        &self,
        username: String,
        password: String,
        receiver: String,
    ) -> Result<Vec<String>, InvalidRequestError> {
        todo!();
    }
}

// # TODO implement stuff
