use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct LatencyTracker {
    latencies: HashMap<String, Duration>,
    last_updated: HashMap<String, Instant>,
}

impl LatencyTracker {
    pub fn new() -> Self {
        Self {
            latencies: HashMap::new(),
            last_updated: HashMap::new(),
        }
    }

    pub fn update(&mut self, node_id: String, latency: Duration) {
        self.latencies.insert(node_id.clone(), latency);
        self.last_updated.insert(node_id, Instant::now());
    }

    pub fn get(&self, node_id: &str) -> Option<Duration> {
        self.latencies.get(node_id).cloned()
    }

    pub fn best_nodes(&self, count: usize) -> Vec<String> {
        let mut nodes: Vec<_> = self.latencies.iter().collect();
        nodes.sort_by_key(|(_, latency)| *latency);

        nodes
            .into_iter()
            .take(count)
            .map(|(id, _)| id.clone())
            .collect()
    }
}
