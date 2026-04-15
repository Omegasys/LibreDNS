use std::collections::HashMap;

use crate::ip_allocator::IpAllocator;
use crate::lease_demo::Lease;

pub struct DhcpNode {
    pub node_id: String,
    pub allocator: IpAllocator,
    pub leases: HashMap<String, Lease>,
}

impl DhcpNode {
    pub fn new(node_id: &str, subnet: u8) -> Self {
        Self {
            node_id: node_id.to_string(),
            allocator: IpAllocator::new(subnet),
            leases: HashMap::new(),
        }
    }

    pub fn assign_ip(&mut self, pubkey: &str) -> Option<Lease> {
        if self.leases.contains_key(pubkey) {
            return self.leases.get(pubkey).cloned();
        }

        let ip = self.allocator.allocate(pubkey)?;

        let lease = Lease::new(pubkey, ip);

        self.leases.insert(pubkey.to_string(), lease.clone());

        Some(lease)
    }

    pub fn validate_leases(&self) {
        for (k, v) in &self.leases {
            println!("Lease check: {} -> {} (valid: {})", k, v.ip, v.is_valid());
        }
    }
}
