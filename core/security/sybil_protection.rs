use std::collections::HashMap;

pub struct SybilProtection {
    node_counts: HashMap<String, usize>, // IP / subnet → count
    max_per_group: usize,
}

impl SybilProtection {
    pub fn new(max_per_group: usize) -> Self {
        Self {
            node_counts: HashMap::new(),
            max_per_group,
        }
    }

    pub fn register_node(&mut self, group_id: &str) -> bool {
        let count = self.node_counts.entry(group_id.to_string()).or_insert(0);

        if *count >= self.max_per_group {
            return false;
        }

        *count += 1;
        true
    }

    pub fn is_allowed(&self, group_id: &str) -> bool {
        self.node_counts
            .get(group_id)
            .map(|&c| c <= self.max_per_group)
            .unwrap_or(true)
    }
}
