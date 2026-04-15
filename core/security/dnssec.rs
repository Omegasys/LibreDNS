use crate::core::records::record::Record;
use crate::core::records::validation::RecordValidator;

pub struct DNSSEC;

impl DNSSEC {
    pub fn validate_chain(records: &[Record]) -> bool {
        // In a real system this would validate a chain of trust
        // For now: validate all records individually
        for record in records {
            if !RecordValidator::validate(record) {
                return false;
            }
        }
        true
    }

    pub fn validate_single(record: &Record) -> bool {
        RecordValidator::validate(record)
    }
}
