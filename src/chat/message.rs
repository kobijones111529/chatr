use serde::{Deserialize, Serialize};

use super::Name;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Sender {
    Anonymous,
    User {
        name: Name,
    },
}

pub type ClientMessage = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerMessage {
    pub sender: Option<Sender>,
    pub msg: String,
}
