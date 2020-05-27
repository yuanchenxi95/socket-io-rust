use crate::socket_io_adaptor::namespace_manager::NamespaceManager;
use crate::socket_io_websocket::app_state::AppState;
use crate::socket_io_websocket::socket_io_websocket_impl::SocketIoWebsocket;
use actix_web::web::Data;
use actix_web::{web, Error, HttpRequest, HttpResponse, Scope};
use actix_web_actors::ws;

async fn ws_index(
    data: web::Data<AppState>,
    r: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    ws::start(SocketIoWebsocket::new(data), &r, stream)
}

pub struct SocketIoServer {}

impl SocketIoServer {
    pub fn get_socket_io_scope(path: &str, app_data: Data<AppState>) -> Scope {
        web::scope(path)
            .app_data(app_data)
            .service(web::resource("").route(web::get().to(ws_index)))
    }

    pub fn get_app_data() -> Data<AppState> {
        let nsm = NamespaceManager::new();
        let app_state = AppState::new(nsm);
        web::Data::new(app_state)
    }
}
