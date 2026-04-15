use crate::core::dhcp::lease::Lease;
use crate::core::crypto::signatures::SignatureUtil;
use ed25519_dalek::VerifyingKey;

pub struct DhcpValidator;

impl DhcpValidator {
    pub fn validate_lease(lease: &Lease, public_key_bytes: &[u8]) -> bool {
        let pub_key = match VerifyingKey::from_bytes(public_key_bytes) {
            Ok(k) => k,
            Err(_) => return false,
        };

        let data = Self::lease_data(lease);

        SignatureUtil::verify(&data, &lease.signature, &pub_key)
    }

    fn lease_data(lease: &Lease) -> Vec<u8> {
        format!(
            "{}:{}:{}",
            lease.ip, lease.client_id, lease.issued_by
        )
        .into_bytes()
    }
}
