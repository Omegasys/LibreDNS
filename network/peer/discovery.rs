use crate::network::peer::peer::Peer;

pub struct Discovery;

impl Discovery {
    pub fn discover_peers() -> Vec<Peer> {
        println!("[Discovery] Finding peers...");

        vec![
            Peer::new("peer-1", "127.0.0.1:9001", "pubkey-1"),
            Peer::new("peer-2", "127.0.0.1:9002", "pubkey-2"),
            Peer::new("peer-3", "127.0.0.1:9003", "pubkey-3"),
        ]
    }
}
