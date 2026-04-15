use crate::core::crypto::encryption::Encryption;
use crate::core::crypto::hashing::Hasher;

pub struct ODoH;

impl ODoH {
    // Blind query (client side)
    pub fn encapsulate(query: &[u8], key: &[u8]) -> (Vec<u8>, [u8; 12]) {
        let (encrypted, nonce) = Encryption::encrypt(key, query);
        (encrypted, nonce)
    }

    // Unblind query (resolver side)
    pub fn decapsulate(
        encrypted: &mut [u8],
        key: &[u8],
        nonce: [u8; 12],
    ) -> Option<Vec<u8>> {
        Encryption::decrypt(key, encrypted, nonce)
    }

    // Optional: hash query to hide exact content patterns
    pub fn hash_query(query: &[u8]) -> Vec<u8> {
        Hasher::hash(query)
    }
}
