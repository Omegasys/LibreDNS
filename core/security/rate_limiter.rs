use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct RateLimiter {
    requests: HashMap<String, Vec<Instant>>,
    limit: usize,
    window: Duration,
}

impl RateLimiter {
    pub fn new(limit: usize, window_secs: u64) -> Self {
        Self {
            requests: HashMap::new(),
            limit,
            window: Duration::from_secs(window_secs),
        }
    }

    pub fn allow(&mut self, client_id: &str) -> bool {
        let now = Instant::now();
        let entries = self.requests.entry(client_id.to_string()).or_default();

        // Remove old requests
        entries.retain(|t| now.duration_since(*t) < self.window);

        if entries.len() >= self.limit {
            return false;
        }

        entries.push(now);
        true
    }
}
