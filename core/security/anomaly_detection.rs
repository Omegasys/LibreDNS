use std::collections::HashMap;

pub struct AnomalyDetector {
    query_counts: HashMap<String, usize>,
    threshold: usize,
}

impl AnomalyDetector {
    pub fn new(threshold: usize) -> Self {
        Self {
            query_counts: HashMap::new(),
            threshold,
        }
    }

    pub fn record_query(&mut self, domain: &str) {
        let count = self.query_counts.entry(domain.to_string()).or_insert(0);
        *count += 1;
    }

    pub fn is_suspicious(&self, domain: &str) -> bool {
        self.query_counts
            .get(domain)
            .map(|&c| c > self.threshold)
            .unwrap_or(false)
    }
}
