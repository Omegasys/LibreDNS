use crate::encryption_layer::EncryptionLayer;

pub struct DoQClient;

impl DoQClient {
    pub fn new() -> Self {
        Self
    }

    pub fn query(&self, domain: &str) {
        let key = "doq-secret-key";

        let encrypted = EncryptionLayer::encrypt(domain, key);

        println!("Sending DoQ QUIC stream...");
        println!("Encrypted payload: {:?}", encrypted);

        let response = self.simulated_server(&encrypted, key);

        println!("Response: {}", response);
    }

    fn simulated_server(&self, data: &[u8], key: &str) -> String {
        let decrypted = EncryptionLayer::decrypt(data, key);
        format!("DoQ resolved: {} -> 10.0.0.3", decrypted)
    }
}
