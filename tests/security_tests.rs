use std::collections::HashSet;

struct ReplayProtector {
    seen: HashSet<u64>,
}

impl ReplayProtector {
    fn new() -> Self {
        Self {
            seen: HashSet::new(),
        }
    }

    fn check(&mut self, nonce: u64) -> bool {
        if self.seen.contains(&nonce) {
            false
        } else {
            self.seen.insert(nonce);
            true
        }
    }
}

#[test]
fn test_replay_protection() {
    let mut protector = ReplayProtector::new();

    assert!(protector.check(123));
    assert!(!protector.check(123)); // replay should fail
}

#[test]
fn test_unique_nonces() {
    let mut protector = ReplayProtector::new();

    for i in 0..1000 {
        assert!(protector.check(i));
    }
}
