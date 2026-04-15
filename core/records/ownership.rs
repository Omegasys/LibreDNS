use crate::core::crypto::signatures::SignatureUtil;
use crate::core::records::record::Record;
use ed25519_dalek::SigningKey;

pub struct Ownership;

impl Ownership {
    pub fn sign_record(record: &mut Record, signing_key: &SigningKey) {
        let data = Self::record_data(record);
        let signature = SignatureUtil::sign(&data, signing_key);
        record.signature = signature;
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
