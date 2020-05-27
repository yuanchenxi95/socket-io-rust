use crate::socket_io_adaptor::chat_room_manager::ChatRoomManager;
use crate::socket_io_adaptor::socket_id_manager::SocketIdManager;
use crate::socket_io_websocket::socket_io_websocket_impl::SocketIoWebsocket;
use crate::socket_io_websocket::socket_message::SocketMessage;
use actix::Addr;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Namespace Error: {}", self.0)]
pub struct NamespaceNameError(&'static str);

//todo create namespaceName
#[derive(Debug)]
pub struct NamespaceName {
    name: String,
}

impl Default for NamespaceName {
    fn default() -> Self {
        Self {
            name: "/".to_string(),
        }
    }
}

impl NamespaceName {
    pub fn try_new(name: &str) -> Result<Self, NamespaceNameError> {
        if name.is_empty() {
            return Err(NamespaceNameError("name cannot be empty."));
        }

        if !name.starts_with('/') {
            return Err(NamespaceNameError("name must start with slash '/'."));
        }

        if name.contains(',') {
            return Err(NamespaceNameError("name cannot contain comma."));
        }

        Ok(Self {
            name: name.to_string(),
        })
    }
}

#[derive(Default, Debug)]
pub struct Namespace {
    name: NamespaceName,
    chat_room_manager: ChatRoomManager,
    socket_id_manager: SocketIdManager,
}

impl Namespace {
    pub fn new(name: NamespaceName) -> Self {
        Self {
            name,
            chat_room_manager: ChatRoomManager::new(),
            socket_id_manager: SocketIdManager::new(),
        }
    }

    pub fn add_socket(&mut self, id: &str, socket_addr: Addr<SocketIoWebsocket>) {
        self.socket_id_manager.add_socket(id, socket_addr);
        // add every socket to a default room that only has himself.
        // https://socket.io/docs/rooms-and-namespaces/
        self.chat_room_manager.add(id, id);
    }

    pub fn join_room(&mut self, id: &str, room_id: &str) {
        self.chat_room_manager.add(id, room_id);
    }

    pub fn leave_room(&mut self, id: &str, room_id: &str) {
        self.chat_room_manager.delete(id, room_id);
    }

    pub fn remove_socket(&mut self, id: &str) {
        self.socket_id_manager.remove_socket_by_id(id);
        self.chat_room_manager.delete_all(id);
    }

    pub fn emit_to(&self, room: &str, event_name: &str, message: &str) {
        self.emit_to_rooms(vec![room], event_name, message);
    }

    pub fn emit_to_rooms(&self, rooms: Vec<&str>, event_name: &str, message: &str) {
        let sid_set = self.chat_room_manager.get_all_sids_from_rooms(rooms);

        for sid in sid_set {
            // todo check sid in manager
            let addr = self.socket_id_manager.get_socket_addr(&sid).unwrap();
            addr.do_send(SocketMessage {
                event_name: event_name.to_string(),
                content: message.to_string(),
            })
        }
    }

    pub fn emit_to_all(&self, event_name: &str, message: &str) {
        let rooms = self.chat_room_manager.get_all_rooms();
        let rooms = rooms.iter().map(|s| s.as_str()).collect();
        self.emit_to_rooms(rooms, event_name, message);
    }

    pub fn get_name(&self) -> &str {
        &self.name.name
    }
}
