use sha2::{Sha256, Digest};

pub struct EncryptionLayer;

impl EncryptionLayer {
    pub fn encrypt(data: &str, key: &str) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        let key_hash = hasher.finalize();

        data.bytes()
            .enumerate()
            .map(|(i, b)| b ^ key_hash[i % key_hash.len()])
            .collect()
    }

    pub fn decrypt(data: &[u8], key: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        let key_hash = hasher.finalize();

        let decrypted: Vec<u8> = data
            .iter()
            .enumerate()
            .map(|(i, b)| b ^ key_hash[i % key_hash.len()])
            .collect();

        String::from_utf8_lossy(&decrypted).to_string()
    }
}
