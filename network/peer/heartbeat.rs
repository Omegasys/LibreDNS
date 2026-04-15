use std::{collections::HashMap, time::{SystemTime, UNIX_EPOCH}};
use crate::network::peer::peer::Peer;

pub struct Heartbeat {
    last_seen: HashMap<String, u64>,
}

impl Heartbeat {
    pub fn new() -> Self {
        Self {
            last_seen: HashMap::new(),
        }
    }

    pub fn ping(&mut self, peer: &Peer) {
        let now = current_time();
        self.last_seen.insert(peer.id.clone(), now);

        println!("[Heartbeat] Pinged {}", peer.id);
    }

    pub fn is_alive(&self, peer_id: &str) -> bool {
        match self.last_seen.get(peer_id) {
            Some(t) => current_time() - t < 30,
            None => false,
        }
    }
}

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
