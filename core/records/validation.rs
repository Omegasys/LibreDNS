use crate::core::records::record::Record;
use crate::core::crypto::signatures::SignatureUtil;
use ed25519_dalek::VerifyingKey;

pub struct RecordValidator;

impl RecordValidator {
    pub fn validate(record: &Record) -> bool {
        // Convert owner bytes into public key
        let pub_key = match VerifyingKey::from_bytes(&record.owner) {
            Ok(pk) => pk,
            Err(_) => return false,
        };

        let data = Self::record_data(record);

        SignatureUtil::verify(&data, &record.signature, &pub_key)
    }

    fn record_data(record: &Record) -> Vec<u8> {
        format!(
            "{}:{:?}:{}:{}",
            record.domain,
            record.record_type,
            record.value,
            record.version
        )
        .into_bytes()
    }
}
