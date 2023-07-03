use std::collections::HashMap;

use actix::prelude::*;
use uuid::Uuid;

use crate::chat::{message::Sender, Name};

#[derive(Message)]
#[rtype(result = "()")]
pub struct HostMessage {
    pub sender: Option<Sender>,
    pub msg: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: Uuid,
    pub msg: String,
    pub name: Option<Name>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub id: Uuid,
    pub addr: Recipient<HostMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
}

pub struct Host {
    sessions: HashMap<Uuid, Recipient<HostMessage>>,
}

enum MessageFilter {
    Include(Uuid),
    Exclude(Uuid),
}

impl Host {
    fn send_message(&self, sender: Option<Sender>, msg: &str, filter: Option<MessageFilter>) {
        self.sessions.iter().for_each(move |session| {
            match filter {
                Some(MessageFilter::Include(id)) => {
                    if id != *session.0 {
                        return;
                    }
                }
                Some(MessageFilter::Exclude(id)) => {
                    if id == *session.0 {
                        return;
                    }
                }
                None => (),
            };
            session.1.do_send(HostMessage {
                sender: sender.clone(),
                msg: msg.to_owned(),
            });
        });
    }
}

impl Default for Host {
    fn default() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }
}

impl Handler<ClientMessage> for Host {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.send_message(
            Some(match msg.name {
                Some(name) => Sender::User { name },
                None => Sender::Anonymous,
            }),
            msg.msg.as_str(),
            Some(MessageFilter::Exclude(msg.id)),
        );
    }
}

impl Handler<Connect> for Host {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        println!("Someone connected! ({})", msg.id);

        self.send_message(None, "Someone connected!", None);

        self.sessions.insert(msg.id, msg.addr);
    }
}

impl Handler<Disconnect> for Host {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
        println!("Disconnecting {}", msg.id);

        self.sessions.remove(&msg.id);
    }
}

impl Actor for Host {
    type Context = Context<Self>;
}
