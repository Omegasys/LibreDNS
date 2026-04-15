use crate::core::crypto::encryption::Encryption;

pub struct OnionLayer {
    pub node_id: String,
    pub encrypted_payload: Vec<u8>,
    pub nonce: [u8; 12],
}

pub struct OnionRouter;

impl OnionRouter {
    pub fn build_onion(
        path: Vec<(String, Vec<u8>)>, // (node_id, key)
        payload: Vec<u8>,
    ) -> Vec<OnionLayer> {
        let mut current_payload = payload;
        let mut layers = Vec::new();

        for (node_id, key) in path.into_iter().rev() {
            let (encrypted, nonce) = Encryption::encrypt(&key, &current_payload);

            let layer = OnionLayer {
                node_id,
                encrypted_payload: encrypted.clone(),
                nonce,
            };

            current_payload = encrypted;
            layers.push(layer);
        }

        layers
    }
}
