use crate::core::protocol::{
    packet::Packet,
    header::Header,
    message::Message,
    errors::ProtocolError,
    version::Version,
};

pub struct Deserializer;

impl Deserializer {
    pub fn deserialize(buf: &[u8]) -> Result<Packet, ProtocolError> {
        if buf.len() < 8 {
            return Err(ProtocolError::BufferTooSmall);
        }

        let version = Version(buf[0]);
        if !version.is_compatible() {
            return Err(ProtocolError::InvalidVersion);
        }

        let message_type = buf[1];
        let flags = u16::from_be_bytes([buf[2], buf[3]]);
        let transaction_id = u32::from_be_bytes([buf[4], buf[5], buf[6], buf[7]]);

        let header = Header {
            version,
            message_type,
            flags,
            transaction_id,
        };

        let mut offset = 8;

        let msg_type = buf[offset];
        offset += 1;

        let message = match msg_type {
            0 => {
                let domain = Self::read_string(buf, &mut offset)?;
                let record_type = buf[offset];
                Message::Query { domain, record_type }
            }
            1 => {
                let domain = Self::read_string(buf, &mut offset)?;
                let value = Self::read_string(buf, &mut offset)?;
                let ttl = u32::from_be_bytes([
                    buf[offset],
                    buf[offset + 1],
                    buf[offset + 2],
                    buf[offset + 3],
                ]);
                Message::Response { domain, value, ttl }
            }
            2 => {
                let code = u16::from_be_bytes([buf[offset], buf[offset + 1]]);
                offset += 2;
                let message = Self::read_string(buf, &mut offset)?;
                Message::Error { code, message }
            }
            _ => return Err(ProtocolError::UnknownMessageType),
        };

        Ok(Packet { header, message })
    }

    fn read_string(buf: &[u8], offset: &mut usize) -> Result<String, ProtocolError> {
        if *offset + 2 > buf.len() {
            return Err(ProtocolError::BufferTooSmall);
        }

        let len = u16::from_be_bytes([buf[*offset], buf[*offset + 1]]) as usize;
        *offset += 2;

        if *offset + len > buf.len() {
            return Err(ProtocolError::BufferTooSmall);
        }

        let s = std::str::from_utf8(&buf[*offset..*offset + len])
            .map_err(|_| ProtocolError::DeserializationError)?
            .to_string();

        *offset += len;
        Ok(s)
    }
}
