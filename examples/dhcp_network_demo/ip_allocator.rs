use sha2::{Digest, Sha256};

pub struct IpAllocator {
    subnet: u8,
}

impl IpAllocator {
    pub fn new(subnet: u8) -> Self {
        Self { subnet }
    }

    pub fn allocate(&self, pubkey: &str) -> Option<String> {
        let mut hasher = Sha256::new();
        hasher.update(pubkey.as_bytes());
        let hash = hasher.finalize();

        let host = hash[0];

        if host == 0 || host == 255 {
            return None;
        }

        Some(format!("{}.{}.{}.{}", self.subnet, 0, 0, host))
    }
}
