use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;

pub struct KeyPair {
    pub signing: SigningKey,
    pub verifying: VerifyingKey,
}

impl KeyPair {
    pub fn generate() -> Self {
        let mut csprng = OsRng;
        let signing = SigningKey::generate(&mut csprng);
        let verifying = signing.verifying_key();

        Self { signing, verifying }
    }

    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.verifying.to_bytes().to_vec()
    }
}
