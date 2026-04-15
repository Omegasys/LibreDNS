use std::collections::HashMap;
use crate::network::peer::peer::Peer;

pub struct PeerManager {
    peers: HashMap<String, Peer>,
}

impl PeerManager {
    pub fn new() -> Self {
        Self {
            peers: HashMap::new(),
        }
    }

    pub fn add_peer(&mut self, peer: Peer) {
        println!("[PeerManager] Added peer {}", peer.id);
        self.peers.insert(peer.id.clone(), peer);
    }

    pub fn get_peer(&self, id: &str) -> Option<&Peer> {
        self.peers.get(id)
    }

    pub fn update_reputation(&mut self, id: &str, delta: f32) {
        if let Some(peer) = self.peers.get_mut(id) {
            peer.reputation += delta;
        }
    }

    pub fn remove_peer(&mut self, id: &str) {
        self.peers.remove(id);
    }

    pub fn list_peers(&self) {
        for (id, peer) in &self.peers {
            println!("[Peer] {} | rep: {}", id, peer.reputation);
        }
    }
}
