use crate::core::crypto::rng::SecureRng;

pub const NONCE_SIZE: usize = 12; // for AES-GCM

pub fn generate_nonce() -> [u8; NONCE_SIZE] {
    let bytes = SecureRng::random_bytes(NONCE_SIZE);
    let mut nonce = [0u8; NONCE_SIZE];
    nonce.copy_from_slice(&bytes);
    nonce
}
