use crate::core::protocol::{header::Header, message::Message};

#[derive(Debug, Clone)]
pub struct Packet {
    pub header: Header,
    pub message: Message,
}

impl Packet {
    pub fn new(header: Header, message: Message) -> Self {
        Self { header, message }
    }
}
