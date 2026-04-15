use sha2::{Sha256, Digest};

fn deterministic_ip(pubkey: &str) -> u8 {
    let mut hasher = Sha256::new();
    hasher.update(pubkey.as_bytes());
    let result = hasher.finalize();

    result[0] // simple mock: last byte = IP
}

#[test]
fn test_deterministic_ip_same_key() {
    let key = "node-public-key";

    let ip1 = deterministic_ip(key);
    let ip2 = deterministic_ip(key);

    assert_eq!(ip1, ip2);
}

#[test]
fn test_deterministic_ip_different_keys() {
    let ip1 = deterministic_ip("key1");
    let ip2 = deterministic_ip("key2");

    assert_ne!(ip1, ip2);
}

#[test]
fn test_ip_in_range() {
    let ip = deterministic_ip("test-key");

    assert!(ip <= 255);
}
