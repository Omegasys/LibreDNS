use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct Lease {
    pub pubkey: String,
    pub ip: String,
    pub timestamp: u64,
    pub ttl: u64,
}

impl Lease {
    pub fn new(pubkey: &str, ip: String) -> Self {
        Self {
            pubkey: pubkey.to_string(),
            ip,
            timestamp: now(),
            ttl: 60,
        }
    }

    pub fn is_valid(&self) -> bool {
        now() < self.timestamp + self.ttl
    }
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
