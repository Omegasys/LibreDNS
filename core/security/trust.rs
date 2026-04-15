use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TrustScore {
    pub score: i32,
}

pub struct TrustManager {
    scores: HashMap<String, TrustScore>,
}

impl TrustManager {
    pub fn new() -> Self {
        Self {
            scores: HashMap::new(),
        }
    }

    pub fn get_score(&self, node_id: &str) -> i32 {
        self.scores.get(node_id).map(|s| s.score).unwrap_or(0)
    }

    pub fn increase(&mut self, node_id: &str, amount: i32) {
        let entry = self.scores.entry(node_id.to_string()).or_insert(TrustScore { score: 0 });
        entry.score += amount;
    }

    pub fn decrease(&mut self, node_id: &str, amount: i32) {
        let entry = self.scores.entry(node_id.to_string()).or_insert(TrustScore { score: 0 });
        entry.score -= amount;
    }

    pub fn is_trusted(&self, node_id: &str, threshold: i32) -> bool {
        self.get_score(node_id) >= threshold
    }
}
