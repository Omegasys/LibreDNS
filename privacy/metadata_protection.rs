use crate::core::crypto::hashing::Hasher;

pub struct MetadataProtection;

impl MetadataProtection {
    // Mask client identity
    pub fn anonymize_id(id: &str) -> String {
        Hasher::hash_hex(id.as_bytes())
    }

    // Strip metadata from packet (placeholder)
    pub fn strip_metadata(packet: &mut Vec<u8>) {
        // In future:
        // - remove headers
        // - encrypt metadata fields
        if packet.len() > 4 {
            packet.drain(0..4);
        }
    }
}
