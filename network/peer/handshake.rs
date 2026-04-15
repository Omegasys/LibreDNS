use crate::network::peer::peer::Peer;

pub struct Handshake;

impl Handshake {
    pub fn perform(peer: &Peer) -> bool {
        println!("[Handshake] Connecting to {}", peer.id);

        if peer.public_key.is_empty() {
            println!("[Handshake] Failed: missing key");
            return false;
        }

        println!("[Handshake] Verified peer {}", peer.id);
        true
    }
}
