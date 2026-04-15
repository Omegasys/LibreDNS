use crate::core::protocol::version::Version;

#[derive(Debug, Clone)]
pub struct Header {
    pub version: Version,
    pub message_type: u8,
    pub flags: u16,
    pub transaction_id: u32,
}

impl Header {
    pub fn new(message_type: u8, flags: u16, transaction_id: u32) -> Self {
        Self {
            version: Version::current(),
            message_type,
            flags,
            transaction_id,
        }
    }
}
