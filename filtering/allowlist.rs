use std::collections::HashSet;

pub struct Allowlist {
    domains: HashSet<String>,
}

impl Allowlist {
    pub fn new() -> Self {
        Self {
            domains: HashSet::new(),
        }
    }

    pub fn insert(&mut self, domain: &str) {
        self.domains.insert(domain.to_string());
    }

    pub fn contains(&self, domain: &str) -> bool {
        self.domains.contains(domain)
    }
}
