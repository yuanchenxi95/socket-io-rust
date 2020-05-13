use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use socket_io_rust::socket_io_adaptor::namespace_manager::NamespaceManager;
use socket_io_rust::socket_io_websocket::app_state::AppState;
use socket_io_rust::socket_io_websocket::socket_io_websocket_impl::SocketIoWebsocket;

#[allow(clippy::let_and_return)]

/// do websocket handshake and start `MyWebSocket` actor
async fn ws_index(
    data: web::Data<AppState>,
    r: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    // println!("{:?}", r);
    let res = ws::start(SocketIoWebsocket::new(data), &r, stream);
    // println!("{:?}", res);
    res
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");

    let mut nsm = NamespaceManager::new();
    nsm.create_namespace("/");
    let app_state = AppState::new(nsm);

    let counter = web::Data::new(app_state);

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            // websocket route
            .service(web::resource("/ws").route(web::get().to(ws_index)))
    })
    // start http server on 127.0.0.1:8080
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
