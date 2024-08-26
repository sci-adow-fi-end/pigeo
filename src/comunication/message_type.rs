use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    Register {
        username: String,
        password: String,
        public_key: String,
    },
    Send {
        username: String,
        password: String,
        message: String,
        receiver: String,
    },
    Receive {
        username: String,
        password: String,
        sender: String,
    },
}
impl Request {
    pub fn encode(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn decode(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Answer {
    Ok,
    BadName,
    BadPwd,
    BadSender,
    BadReceiver,
    Messages(Vec<String>),
}

impl Answer {
    pub fn encode(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn decode(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }
}
