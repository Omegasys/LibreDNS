use std::collections::HashSet;

pub struct IpPool {
    available: HashSet<String>,
    allocated: HashSet<String>,
}

impl IpPool {
    pub fn new(ips: Vec<String>) -> Self {
        Self {
            available: ips.into_iter().collect(),
            allocated: HashSet::new(),
        }
    }

    pub fn allocate(&mut self) -> Option<String> {
        if let Some(ip) = self.available.iter().next().cloned() {
            self.available.remove(&ip);
            self.allocated.insert(ip.clone());
            Some(ip)
        } else {
            None
        }
    }

    pub fn release(&mut self, ip: &str) {
        if self.allocated.remove(ip) {
            self.available.insert(ip.to_string());
        }
    }
}
