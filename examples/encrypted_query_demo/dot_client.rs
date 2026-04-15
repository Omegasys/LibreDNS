use crate::encryption_layer::EncryptionLayer;

pub struct DoTClient;

impl DoTClient {
    pub fn new() -> Self {
        Self
    }

    pub fn query(&self, domain: &str) {
        let key = "dot-secret-key";

        let encrypted = EncryptionLayer::encrypt(domain, key);

        println!("Sending DoT TLS packet...");
        println!("Encrypted payload: {:?}", encrypted);

        let response = self.simulated_server(&encrypted, key);

        println!("Response: {}", response);
    }

    fn simulated_server(&self, data: &[u8], key: &str) -> String {
        let decrypted = EncryptionLayer::decrypt(data, key);
        format!("DoT resolved: {} -> 10.0.0.2", decrypted)
    }
}
