use std::{error::Error, io::Result};

use openssl::error::Error;
use postgres::Client;
use serde_json::Result;

struct dao {
    db_client: Client,
}
impl dao {
    fn new() -> Self {}

    fn save_user(username: String, password: String, pubkey: String) -> Result<(), Error> {}

    fn validate_credentials(username: String, password: String)->Result<(),Error>{}

    fn save_message(
        username: String,
        password: String,
        message: String,
        receiver: String,
    ) -> Result<(), Error> {
    }

    fn get_messages_by_receiver(username: String, password: String, receiver: String)->Result<Vec<String>,Error> 
}

// # TODO implement stuff
