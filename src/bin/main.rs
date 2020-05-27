use actix_web::{App, HttpServer};
use socket_io_rust::socket_io::socket_io_server::SocketIoServer;

#[allow(clippy::let_and_return)]
// /// do websocket handshake and start `MyWebSocket` actor
// async fn ws_index(
//     data: web::Data<AppState>,
//     r: HttpRequest,
//     stream: web::Payload,
// ) -> Result<HttpResponse, Error> {
//     ws::start(SocketIoWebsocket::new(data), &r, stream)
// }
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");

    // let mut nsm = NamespaceManager::new();
    // nsm.create_namespace("/").unwrap();
    // let app_state = AppState::new(nsm);
    //
    // let counter = web::Data::new(app_state);

    // HttpServer::new(move || {
    //     App::new()
    //         .app_data(counter.clone())
    //         // websocket route
    //         .service(web::resource("/ws").route(web::get().to(ws_index)))
    // });

    let app_data = SocketIoServer::get_app_data();
    HttpServer::new(move || {
        App::new().service(SocketIoServer::get_socket_io_scope("/ws", app_data.clone()))
    })
    // start http server on 127.0.0.1:8080
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
