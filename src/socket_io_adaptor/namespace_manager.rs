use crate::socket_io_adaptor::io_adaptor::IoAdaptor;
use std::collections::HashMap;

pub struct NamespaceManager {
    nsp_adaptor_map: HashMap<&'static str, IoAdaptor>,
}

impl Default for NamespaceManager {
    fn default() -> Self {
        Self {
            nsp_adaptor_map: HashMap::new(),
        }
    }
}

impl NamespaceManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_namespace(&mut self, nsp: &'static str) {
        self.nsp_adaptor_map.insert(nsp, IoAdaptor::new());
    }

    pub fn get_adaptor_mut(&mut self, nsp: &'static str) -> Option<&mut IoAdaptor> {
        self.nsp_adaptor_map.get_mut(nsp)
    }

    pub fn get_adaptor(&self, nsp: &'static str) -> Option<&IoAdaptor> {
        self.nsp_adaptor_map.get(nsp)
    }
}
