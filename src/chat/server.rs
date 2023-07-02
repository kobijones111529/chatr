use std::collections::HashMap;

use actix::prelude::*;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ServerMessage(pub String);

pub struct ClientMessage {
    pub id: Uuid,
    pub msg: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub id: Uuid,
    pub addr: Recipient<ServerMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
}

pub struct Server {
    sessions: HashMap<Uuid, Recipient<ServerMessage>>,
}

impl Server {
    fn send_message(&self, msg: &str) {
        self.sessions.iter().for_each(move |session| {
            session.1.do_send(ServerMessage(msg.to_owned()));
        });
    }
}

impl Default for Server {
    fn default() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }
}

impl Handler<Connect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        println!("Someone connected! ({})", msg.id);

        self.send_message("Someone connected!");

        self.sessions.insert(msg.id, msg.addr);
    }
}

impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, ctx: &mut Self::Context) -> Self::Result {
        println!("Disconnecting {}", msg.id);

        self.sessions.remove(&msg.id);
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}
