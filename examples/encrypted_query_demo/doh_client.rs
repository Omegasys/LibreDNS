use crate::encryption_layer::EncryptionLayer;

pub struct DoHClient;

impl DoHClient {
    pub fn new() -> Self {
        Self
    }

    pub fn query(&self, domain: &str) {
        let key = "doh-secret-key";

        let encrypted = EncryptionLayer::encrypt(domain, key);

        println!("Sending DoH HTTPS request...");
        println!("Encrypted payload: {:?}", encrypted);

        let response = self.simulated_server(&encrypted, key);

        println!("Response: {}", response);
    }

    fn simulated_server(&self, data: &[u8], key: &str) -> String {
        let decrypted = EncryptionLayer::decrypt(data, key);
        format!("DoH resolved: {} -> 10.0.0.1", decrypted)
    }
}
