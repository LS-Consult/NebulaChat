use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Client {
    known_peers: Arc<Mutex<HashMap<String, super::peer::Peer>>>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            known_peers: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
