#[derive(Debug, Clone)]
pub struct Request {
    pub client_id: String,
    pub requested_ip: String,
    pub server_id: String,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Ack {
    pub lease_id: String,
    pub ip: String,
    pub expiration: u64,
    pub signature: Vec<u8>,
}
