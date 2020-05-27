use crate::random_id_generator::RandomIdGenerator;
use crate::socket_io_websocket::app_state::AppState;
use crate::socket_io_websocket::socket_message::SocketMessage;
use actix::{Actor, ActorContext, AsyncContext, Handler, StreamHandler};
use actix_web::web;
use actix_web_actors::ws;
use std::io;
use std::time::{Duration, Instant};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// websocket connection is long running connection, it easier
/// to handle with an actor
#[derive(Debug)]
pub struct SocketIoWebsocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
    global_state: web::Data<AppState>,
    id: String,
    namespace: String,
}

impl Actor for SocketIoWebsocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.start_web_socket(ctx);
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        self.stop_web_socket();
    }
}

impl Handler<SocketMessage> for SocketIoWebsocket {
    type Result = Result<(), io::Error>;

    fn handle(&mut self, msg: SocketMessage, ctx: &mut Self::Context) -> Self::Result {
        let content = format!("{},{}", msg.event_name, msg.content);
        ctx.text(content);
        Ok(())
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for SocketIoWebsocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // process websocket messages
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                let namespace_manager = self.global_state.namespace_manager.read().unwrap();
                let adaptor = namespace_manager.get_adaptor("/").unwrap();
                adaptor.emit_to_all("hello", &text);
            }
            Ok(ws::Message::Binary(_)) => {
                // todo unsupported binary
                ctx.stop();
            },
            Ok(ws::Message::Close(_)) => {
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

#[allow(dead_code)]
impl SocketIoWebsocket {
    pub fn new(data: web::Data<AppState>, namespace: &str) -> Self {
        Self {
            hb: Instant::now(),
            global_state: data,
            id: RandomIdGenerator::get_random_uuid(),
            namespace: namespace.to_string(),
        }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }

    fn start_web_socket(&mut self, ctx: &mut <Self as Actor>::Context) {
        self.add_socket_to_adaptor(ctx);
        self.hb(ctx);
    }


    fn stop_web_socket(&mut self) {
        self.remove_socket_from_adaptor();
    }


    fn add_socket_to_adaptor(&mut self, ctx: &mut <Self as Actor>::Context) {
        let mut namespace_manager = self.global_state.namespace_manager.write().unwrap();
        let adaptor = namespace_manager.get_adaptor_mut(&self.namespace).unwrap();
        adaptor.add_socket(&self.id, ctx.address());
    }

    fn remove_socket_from_adaptor(&mut self) {
        let mut namespace_manager = self.global_state.namespace_manager.write().unwrap();
        let adaptor = namespace_manager.get_adaptor_mut("/").unwrap();
        adaptor.remove_socket(&self.id);
    }
}
