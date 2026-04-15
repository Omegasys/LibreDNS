#[derive(Debug, Clone)]
pub struct Peer {
    pub id: String,
    pub address: String,
    pub public_key: String,
    pub reputation: f32,
    pub last_seen: u64,
    pub connected: bool,
}

impl Peer {
    pub fn new(id: &str, address: &str, public_key: &str) -> Self {
        Self {
            id: id.to_string(),
            address: address.to_string(),
            public_key: public_key.to_string(),
            reputation: 1.0,
            last_seen: 0,
            connected: false,
        }
    }
}
