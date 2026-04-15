use std::collections::HashMap;
use crate::core::records::record::Record;

pub struct RecordStore {
    records: HashMap<String, Vec<Record>>,
}

impl RecordStore {
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
        }
    }

    pub fn insert(&mut self, record: Record) {
        let key = record.domain.clone();

        self.records
            .entry(key)
            .or_insert_with(Vec::new)
            .push(record);
    }

    pub fn get(&mut self, domain: &str) -> Option<Vec<Record>> {
        if let Some(records) = self.records.get_mut(domain) {
            // Remove expired records
            records.retain(|r| !r.ttl.is_expired());

            if records.is_empty() {
                return None;
            }

            return Some(records.clone());
        }
        None
    }

    pub fn remove(&mut self, domain: &str) {
        self.records.remove(domain);
    }
}
