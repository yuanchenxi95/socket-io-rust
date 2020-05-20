use crate::socket_io_adaptor::namespace::Namespace;
use std::collections::HashMap;

pub struct NamespaceManager {
    nsp_adaptor_map: HashMap<&'static str, Namespace>,
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
        self.nsp_adaptor_map
            .insert(nsp, Namespace::new(nsp.to_string()));
    }

    pub fn get_adaptor_mut(&mut self, nsp: &'static str) -> Option<&mut Namespace> {
        self.nsp_adaptor_map.get_mut(nsp)
    }

    pub fn get_adaptor(&self, nsp: &'static str) -> Option<&Namespace> {
        self.nsp_adaptor_map.get(nsp)
    }
}
