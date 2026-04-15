use crate::core::records::record::Record;

pub struct Versioning;

impl Versioning {
    pub fn resolve_conflict(records: Vec<Record>) -> Option<Record> {
        records.into_iter().max_by_key(|r| r.version)
    }

    pub fn increment_version(record: &mut Record) {
        record.version += 1;
    }
}
