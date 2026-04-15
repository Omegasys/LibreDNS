use std::collections::HashMap;
use std::time::{Instant, Duration};

#[derive(Clone)]
pub struct CacheEntry {
    pub value: String,
    pub expires_at: Instant,
}

pub struct ResolverCache {
    store: HashMap<String, CacheEntry>,
}

impl ResolverCache {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: &str) -> Option<String> {
        if let Some(entry) = self.store.get(key) {
            if Instant::now() < entry.expires_at {
                return Some(entry.value.clone());
            }
        }
        self.store.remove(key);
        None
    }

    pub fn insert(&mut self, key: String, value: String, ttl: u64) {
        let entry = CacheEntry {
            value,
            expires_at: Instant::now() + Duration::from_secs(ttl),
        };
        self.store.insert(key, entry);
    }
}
