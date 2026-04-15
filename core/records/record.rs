use crate::core::records::record_types::RecordType;
use crate::core::records::ttl::TTL;

#[derive(Debug, Clone)]
pub struct Record {
    pub domain: String,
    pub record_type: RecordType,
    pub value: String,
    pub ttl: TTL,
    pub version: u64,
    pub owner: Vec<u8>,       // public key
    pub signature: Vec<u8>,   // signed record
}

impl Record {
    pub fn new(
        domain: String,
        record_type: RecordType,
        value: String,
        ttl_seconds: u64,
        owner: Vec<u8>,
    ) -> Self {
        Self {
            domain,
            record_type,
            value,
            ttl: TTL::new(ttl_seconds),
            version: 0,
            owner,
            signature: Vec::new(),
        }
    }
}
