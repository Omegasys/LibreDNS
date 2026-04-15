use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct TTL {
    created_at: Instant,
    duration: Duration,
}

impl TTL {
    pub fn new(seconds: u64) -> Self {
        Self {
            created_at: Instant::now(),
            duration: Duration::from_secs(seconds),
        }
    }

    pub fn is_expired(&self) -> bool {
        Instant::now() > self.created_at + self.duration
    }

    pub fn remaining(&self) -> Option<Duration> {
        if self.is_expired() {
            None
        } else {
            Some((self.created_at + self.duration) - Instant::now())
        }
    }
}
