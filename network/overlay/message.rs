#[derive(Debug, Clone)]
pub enum MessageType {
    DnsQuery,
    DnsResponse,
    DhcpRequest,
    DhcpLease,
    Gossip,
    DhtPut,
    DhtGet,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub id: String,
    pub from: String,
    pub to: Option<String>,
    pub msg_type: MessageType,
    pub payload: Vec<u8>,
    pub hop: u8,
}

impl Message {
    pub fn new(id: &str, from: &str, msg_type: MessageType, payload: Vec<u8>) -> Self {
        Self {
            id: id.to_string(),
            from: from.to_string(),
            to: None,
            msg_type,
            payload,
            hop: 0,
        }
    }
}
