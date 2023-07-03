use crate::chat::{
    message::{ClientMessage, ServerMessage},
    server::host,
    Name,
};

use super::host::{Connect, Disconnect, Host, HostMessage};
use actix::*;
use actix_web_actors::ws::{self, WebsocketContext};
use uuid::Uuid;

pub struct Session {
    id: Option<Uuid>,
    addr: Addr<Host>,
    name: Option<Name>,
}

impl Session {
    pub fn new(addr: Addr<Host>) -> Self {
        Self {
            id: None,
            addr,
            name: None,
        }
    }
}

impl Handler<HostMessage> for Session {
    type Result = ();

    fn handle(&mut self, msg: HostMessage, ctx: &mut Self::Context) -> Self::Result {
        let msg = ServerMessage {
            sender: msg.sender,
            msg: msg.msg,
        };
        match serde_json::to_string(&msg) {
            Ok(msg) => ctx.text(msg),
            Err(_) => (),
        };
    }
}

impl Actor for Session {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        let id = Uuid::new_v4();
        self.addr
            .send(Connect {
                id,
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(move |res, act, ctx| {
                if res.is_err() {
                    ctx.stop();
                } else {
                    act.id = Some(id);
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        match self.id {
            Some(id) => self.addr.do_send(Disconnect { id }),
            None => (),
        }
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        use ws::Message::*;

        let msg = match item {
            Ok(msg) => msg,
            Err(_) => {
                return;
            }
        };

        match msg {
            Ping(msg) => ctx.pong(&msg),
            Pong(_) => (),
            Text(text) => {
                match self.id {
                    Some(id) => {
                        let msg = serde_json::from_str::<ClientMessage>(text.to_string().as_str());
                        match msg {
                            Ok(msg) => {
                                let msg = msg.trim();
                                if msg.starts_with('/') {
                                    let msg: Vec<&str> = msg.splitn(2, ' ').collect();
                                    match msg[0] {
                                        "/name" => match msg.get(1) {
                                            Some(name) => match Name::new(name) {
                                                Some(name) => {
                                                    let msg = ServerMessage {
                                                        sender: None,
                                                        msg: format!(
                                                            "Hello, {}!",
                                                            name.clone().value()
                                                        ),
                                                    };
                                                    self.name = Some(name);
                                                    match serde_json::to_string::<ServerMessage>(
                                                        &msg,
                                                    ) {
                                                        Ok(json) => ctx.text(json),
                                                        Err(_) => (),
                                                    }
                                                }
                                                None => println!("Invalid name"),
                                            },
                                            None => match &self.name {
                                                Some(name) => println!("Your name is {}", name.0),
                                                None => println!("You haven't set a name yet"),
                                            },
                                        },
                                        _ => {
                                            println!("Invalid command");
                                        }
                                    }
                                } else {
                                    self.addr.do_send(host::ClientMessage {
                                        id,
                                        msg: msg.to_owned(),
                                        name: self.name.clone(),
                                    })
                                }
                            }
                            Err(_) => (),
                        };
                    }
                    None => (),
                };
            }
            Binary(_) => (),
            Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            Continuation(_) => {
                ctx.stop();
            }
            Nop => (),
        };
    }
}
