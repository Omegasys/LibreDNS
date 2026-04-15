#[derive(Debug, Clone)]
pub struct Discover {
    pub client_id: String,
    pub public_key: Vec<u8>,
    pub nonce: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Offer {
    pub ip: String,
    pub lease_time: u64,
    pub server_id: String,
    pub signature: Vec<u8>,
}
