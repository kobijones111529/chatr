use super::server::{Connect, Disconnect, ServerMessage, Server};
use actix::*;
use actix_web_actors::ws::{self, WebsocketContext};
use uuid::Uuid;

pub struct Session {
    id: Option<Uuid>,
    addr: Addr<Server>,
}

impl Session {
    pub fn new(addr: Addr<Server>) -> Self {
        Self { id: None, addr }
    }
}

impl Handler<ServerMessage> for Session {
    type Result = ();

    fn handle(&mut self, msg: ServerMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
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

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
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
            Text(text) => ctx.text(text),
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
