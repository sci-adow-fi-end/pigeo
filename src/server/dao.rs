use crate::{client::client, server::invalid_request_error::InvalidRequestError};
use log::{info, warn};
use postgres::{Client, Error, GenericClient, NoTls};
use std::result::Result;

pub struct DAO {
    db_client: Client,
}
impl DAO {
    pub fn init_connection() -> Option<Self> {
        let connection = Client::connect(
            "host=localhost user=postgres password=Mind dbname=pigeo",
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
                            Ok(_res) => {
                                info!("database initialized succesfully");
                                Some(DAO { db_client: client })
                            }
                        }
                    }
                }
            }
        };
    }

    pub fn is_name_present(&mut self, username: String) -> Result<bool, InvalidRequestError> {
        match self.db_client.query(
            "SELECT username 
                    FROM users
                    WHERE username = $1",
            &[&username],
        ) {
            Ok(rows) => {
                let available: bool = !rows.is_empty();
                Ok(available)
            }
            Err(_e) => Err(InvalidRequestError::DatabaseError),
        }
    }

    pub fn save_user(
        &mut self,
        username: String,
        password: String,
        pubkey: String,
    ) -> Result<(), InvalidRequestError> {
        match self.db_client.execute(
            "INSERT INTO users (username, password, pubkey) VALUES ($1, $2, $3)",
            &[&username, &password, &pubkey],
        ) {
            Ok(_res) => Ok(()),
            Err(_e) => Err(InvalidRequestError::DatabaseError),
        }
    }

    fn validate_credentials(username: String, password: String) -> Result<(), InvalidRequestError> {
        todo!()
    }

    pub fn save_message(
        &mut self,
        message: String,
        sender: String,
        receiver: String,
    ) -> Result<(), InvalidRequestError> {
        match self.db_client.execute(
            "INSERT INTO messages (message, sender, receiver) VALUES ($1, $2, $3)",
            &[&message, &sender, &receiver],
        ) {
            Ok(_res) => Ok(()),
            Err(_e) => Err(InvalidRequestError::DatabaseError),
        }
    }

    pub fn get_messages_by_sender_receiver(
        &mut self,
        sender: String,
        receiver: String,
    ) -> Result<Vec<String>, InvalidRequestError> {
        match self.db_client.query(
            "SELECT message 
                    FROM messages
                    WHERE sender = $1 AND receiver = $2",
            &[&sender, &receiver],
        ) {
            Ok(rows) => {
                let messages: Vec<String> = rows.iter().map(|row| row.get("message")).collect();
                Ok(messages)
            }
            Err(_e) => Err(InvalidRequestError::DatabaseError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DAO;

    #[test]
    fn test_init_connection() {
        assert!(DAO::init_connection().is_some())
    }
}

// # TODO implement stuff
