use std::collections::HashMap;

pub struct Dht {
    storage: HashMap<String, Vec<u8>>,
}

impl Dht {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    pub fn put(&mut self, key: &str, value: Vec<u8>) {
        self.storage.insert(key.to_string(), value);
    }

    pub fn get(&self, key: &str) -> Option<&Vec<u8>> {
        self.storage.get(key)
    }

    pub fn contains(&self, key: &str) -> bool {
        self.storage.contains_key(key)
    }
}
