use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Lease {
    pub ip: String,
    pub client_id: String,
    pub issued_by: String,
    pub start: Instant,
    pub duration: Duration,
    pub signature: Vec<u8>,
}

impl Lease {
    pub fn new(ip: String, client_id: String, issued_by: String, duration_secs: u64) -> Self {
        Self {
            ip,
            client_id,
            issued_by,
            start: Instant::now(),
            duration: Duration::from_secs(duration_secs),
            signature: Vec::new(),
        }
    }

    pub fn is_expired(&self) -> bool {
        Instant::now() > self.start + self.duration
    }
}
