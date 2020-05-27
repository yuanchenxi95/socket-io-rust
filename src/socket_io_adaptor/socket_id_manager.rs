use crate::socket_io_websocket::socket_io_websocket_impl::SocketIoWebsocket;
use actix::Addr;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct SocketIdManager {
    id_socket_map: HashMap<String, Addr<SocketIoWebsocket>>,
}

impl SocketIdManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_socket(&mut self, id: &str, socket_addr: Addr<SocketIoWebsocket>) {
        self.id_socket_map.insert(id.to_string(), socket_addr);
    }

    pub fn get_socket_addr(&self, id: &str) -> Option<&Addr<SocketIoWebsocket>> {
        self.id_socket_map.get(id)
    }

    pub fn remove_socket_by_id(&mut self, id: &str) -> Option<Addr<SocketIoWebsocket>> {
        self.id_socket_map.remove(id)
    }
}
