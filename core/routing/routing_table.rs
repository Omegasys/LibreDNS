use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Peer {
    pub id: String,
    pub address: String,
}

pub struct RoutingTable {
    peers: HashMap<String, Peer>,
}

impl RoutingTable {
    pub fn new() -> Self {
        Self {
            peers: HashMap::new(),
        }
    }

    pub fn add_peer(&mut self, peer: Peer) {
        self.peers.insert(peer.id.clone(), peer);
    }

    pub fn remove_peer(&mut self, id: &str) {
        self.peers.remove(id);
    }

    pub fn get_peer(&self, id: &str) -> Option<&Peer> {
        self.peers.get(id)
    }

    pub fn all_peers(&self) -> Vec<Peer> {
        self.peers.values().cloned().collect()
    }
}
