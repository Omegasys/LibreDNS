use crate::core::protocol::{
    packet::Packet,
    message::Message,
    errors::ProtocolError,
};

pub struct Serializer;

impl Serializer {
    pub fn serialize(packet: &Packet) -> Result<Vec<u8>, ProtocolError> {
        let mut buf = Vec::new();

        // Header
        buf.push(packet.header.version.0);
        buf.push(packet.header.message_type);
        buf.extend_from_slice(&packet.header.flags.to_be_bytes());
        buf.extend_from_slice(&packet.header.transaction_id.to_be_bytes());

        // Message
        match &packet.message {
            Message::Query { domain, record_type } => {
                buf.push(0);
                Self::write_string(&mut buf, domain);
                buf.push(*record_type);
            }
            Message::Response { domain, value, ttl } => {
                buf.push(1);
                Self::write_string(&mut buf, domain);
                Self::write_string(&mut buf, value);
                buf.extend_from_slice(&ttl.to_be_bytes());
            }
            Message::Error { code, message } => {
                buf.push(2);
                buf.extend_from_slice(&code.to_be_bytes());
                Self::write_string(&mut buf, message);
            }
        }

        Ok(buf)
    }

    fn write_string(buf: &mut Vec<u8>, s: &str) {
        let bytes = s.as_bytes();
        let len = bytes.len() as u16;
        buf.extend_from_slice(&len.to_be_bytes());
        buf.extend_from_slice(bytes);
    }
}
