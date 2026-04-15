use std::collections::HashSet;
use std::time::{Duration, Instant};

pub struct ReplayProtection {
    seen_nonces: HashSet<Vec<u8>>,
    timestamps: HashSet<u64>,
    window: Duration,
}

impl ReplayProtection {
    pub fn new() -> Self {
        Self {
            seen_nonces: HashSet::new(),
            timestamps: HashSet::new(),
            window: Duration::from_secs(30),
        }
    }

    pub fn validate_nonce(&mut self, nonce: &[u8]) -> bool {
        if self.seen_nonces.contains(nonce) {
            return false;
        }

        self.seen_nonces.insert(nonce.to_vec());
        true
    }

    pub fn validate_timestamp(&mut self, timestamp: u64) -> bool {
        let now = Instant::now();

        // Simple check (you can improve with real clock sync)
        if self.timestamps.contains(&timestamp) {
            return false;
        }

        self.timestamps.insert(timestamp);
        true
    }
}
