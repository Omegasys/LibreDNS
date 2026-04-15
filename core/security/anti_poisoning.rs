use std::collections::HashMap;
use crate::core::records::record::Record;

pub struct AntiPoisoning {
    seen_records: HashMap<String, u64>, // domain -> version
}

impl AntiPoisoning {
    pub fn new() -> Self {
        Self {
            seen_records: HashMap::new(),
        }
    }

    pub fn validate(&mut self, record: &Record) -> bool {
        let key = record.domain.clone();

        if let Some(prev_version) = self.seen_records.get(&key) {
            if record.version < *prev_version {
                // Older version → possible poisoning attempt
                return false;
            }
        }

        self.seen_records.insert(key, record.version);
        true
    }
}
