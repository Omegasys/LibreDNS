#[derive(Clone)]
pub struct Config {
    pub resolver_address: String,
    pub use_encryption: bool,
    pub use_onion_routing: bool,
    pub timeout_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            resolver_address: "127.0.0.1:5353".to_string(),
            use_encryption: true,
            use_onion_routing: true,
            timeout_ms: 5000,
        }
    }
}
