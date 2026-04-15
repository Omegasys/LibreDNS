use ring::aead::{Aad, LessSafeKey, UnboundKey, AES_256_GCM, Nonce};
use crate::core::crypto::nonce::{generate_nonce, NONCE_SIZE};

pub struct Encryption;

impl Encryption {
    pub fn encrypt(key_bytes: &[u8], plaintext: &[u8]) -> (Vec<u8>, [u8; NONCE_SIZE]) {
        let unbound_key = UnboundKey::new(&AES_256_GCM, key_bytes).unwrap();
        let key = LessSafeKey::new(unbound_key);

        let mut in_out = plaintext.to_vec();
        let nonce_bytes = generate_nonce();
        let nonce = Nonce::assume_unique_for_key(nonce_bytes);

        key.seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
            .expect("encryption failed");

        (in_out, nonce_bytes)
    }

    pub fn decrypt(
        key_bytes: &[u8],
        ciphertext: &mut [u8],
        nonce_bytes: [u8; NONCE_SIZE],
    ) -> Option<Vec<u8>> {
        let unbound_key = UnboundKey::new(&AES_256_GCM, key_bytes).ok()?;
        let key = LessSafeKey::new(unbound_key);

        let nonce = Nonce::assume_unique_for_key(nonce_bytes);

        let result = key
            .open_in_place(nonce, Aad::empty(), ciphertext)
            .ok()?;

        Some(result.to_vec())
    }
}
