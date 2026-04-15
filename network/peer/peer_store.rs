use std::collections::HashMap;
use crate::network::peer::peer::Peer;

pub struct PeerStore {
    storage: HashMap<String, Peer>,
}

impl PeerStore {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    pub fn save(&mut self, peer: Peer) {
        self.storage.insert(peer.id.clone(), peer);
    }

    pub fn load(&self, id: &str) -> Option<Peer> {
        self.storage.get(id).cloned()
    }

    pub fn all(&self) -> Vec<Peer> {
        self.storage.values().cloned().collect()
    }
}
