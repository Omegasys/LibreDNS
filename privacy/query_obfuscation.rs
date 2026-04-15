use crate::core::crypto::rng::SecureRng;

pub struct QueryObfuscator;

impl QueryObfuscator {
    pub fn obfuscate(query: &[u8]) -> Vec<u8> {
        let mut result = query.to_vec();

        // Add random noise
        let noise_len = (SecureRng::random_u32() % 16) as usize;
        let noise = SecureRng::random_bytes(noise_len);

        result.extend(noise);
        result
    }

    pub fn deobfuscate(data: &[u8], original_len: usize) -> Vec<u8> {
        data[..original_len].to_vec()
    }
}
