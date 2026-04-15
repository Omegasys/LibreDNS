use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Lease {
    pub pubkey: String,
    pub ip: String,
    pub timestamp: u64,
    pub ttl: u64,
    pub signature: String,
}

pub fn inspect_lease(lease: &Lease) {
    println!("=== DHCP Lease Inspector ===");
    println!("Public Key: {}", lease.pubkey);
    println!("IP Address: {}", lease.ip);
    println!("Timestamp: {}", lease.timestamp);
    println!("TTL: {}", lease.ttl);
    println!("Signature: {}", lease.signature);

    let valid = validate_lease(lease);
    println!("Valid: {}", valid);
}

pub fn validate_lease(lease: &Lease) -> bool {
    // Placeholder validation logic
    // Replace with:
    // - Signature verification
    // - Deterministic IP check
    // - Expiry check

    if lease.ip.is_empty() || lease.pubkey.is_empty() {
        return false;
    }

    let current_time = current_time();
    lease.timestamp + lease.ttl > current_time
}

fn current_time() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
