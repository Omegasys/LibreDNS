use x25519_dalek::{EphemeralSecret, PublicKey};
use rand_core::OsRng;

pub struct KeyExchange;

impl KeyExchange {
    pub fn generate_keypair() -> (EphemeralSecret, PublicKey) {
        let secret = EphemeralSecret::random_from_rng(OsRng);
        let public = PublicKey::from(&secret);
        (secret, public)
    }

    pub fn compute_shared_secret(
        secret: EphemeralSecret,
        peer_public: PublicKey,
    ) -> [u8; 32] {
        let shared = secret.diffie_hellman(&peer_public);
        *shared.as_bytes()
    }
}
