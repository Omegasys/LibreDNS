use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};

pub struct SignatureUtil;

impl SignatureUtil {
    pub fn sign(data: &[u8], key: &SigningKey) -> Vec<u8> {
        let sig: Signature = key.sign(data);
        sig.to_bytes().to_vec()
    }

    pub fn verify(data: &[u8], sig_bytes: &[u8], public_key: &VerifyingKey) -> bool {
        if let Ok(sig) = Signature::from_bytes(sig_bytes) {
            public_key.verify(data, &sig).is_ok()
        } else {
            false
        }
    }
}
