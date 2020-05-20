use crate::socket_io_adaptor::namespace::{Namespace, NamespaceName, NamespaceNameError};
use std::collections::HashMap;

pub struct NamespaceManager {
    nsp_adaptor_map: HashMap<String, Namespace>,
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

    pub fn create_namespace(&mut self, nsp: &str) -> Result<(), NamespaceNameError> {
        let name = NamespaceName::try_new(nsp)?;
        let namespace = Namespace::new(name);
        self.nsp_adaptor_map.insert(nsp.to_string(), namespace);
        Ok(())
    }

    pub fn get_adaptor_mut(&mut self, nsp: &str) -> Option<&mut Namespace> {
        self.nsp_adaptor_map.get_mut(nsp)
    }

    pub fn get_adaptor(&self, nsp: &str) -> Option<&Namespace> {
        self.nsp_adaptor_map.get(nsp)
    }
}

#[cfg(test)]
mod tests {
    use crate::socket_io_adaptor::namespace_manager::NamespaceManager;

    #[test]
    pub fn manager_test() {
        let mut m = NamespaceManager::new();

        let s = String::from("/hello");
        assert!(m.create_namespace(&s).is_ok());
        assert!(m.get_adaptor(&s).is_some());
    }
}
