use std::time::{Instant, Duration};

#[derive(Clone, Debug)]
pub struct QueryContext {
    pub domain: String,
    pub record_type: RecordType,
    pub start_time: Instant,
    pub timeout: Duration,
    pub retries: u8,
}

#[derive(Clone, Debug)]
pub enum RecordType {
    A,
    AAAA,
    TXT,
    CNAME,
}

impl QueryContext {
    pub fn new(domain: String, record_type: RecordType) -> Self {
        Self {
            domain,
            record_type,
            start_time: Instant::now(),
            timeout: Duration::from_secs(5),
            retries: 3,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.start_time.elapsed() > self.timeout
    }
}
