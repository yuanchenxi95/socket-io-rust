use crate::socket_io_adaptor::namespace_manager::NamespaceManager;
use std::sync::RwLock;

// pub struct AppState {
//     pub(crate) all_socket: RwLock<HashSet<Addr<SocketIoWebsocket>>>,
// }
pub struct AppState {
    pub namespace_manager: RwLock<NamespaceManager>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            namespace_manager: RwLock::new(NamespaceManager::new()),
        }
    }
}

impl AppState {
    pub fn new(nsm: NamespaceManager) -> Self {
        Self {
            namespace_manager: RwLock::new(nsm),
        }
    }
}
