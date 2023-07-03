pub mod host;
pub mod session;

use actix::Addr;
use actix_web::{
    web::{Data, Payload},
    Error, HttpRequest, HttpResponse,
};
use actix_web_actors::ws;

use host::Host;
use session::Session;

pub async fn ws(
    req: HttpRequest,
    stream: Payload,
    server: Data<Addr<Host>>,
) -> Result<HttpResponse, Error> {
    ws::start(Session::new(server.get_ref().clone()), &req, stream)
}
