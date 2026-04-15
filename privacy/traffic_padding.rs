use crate::core::crypto::rng::SecureRng;

pub struct TrafficPadding;

impl TrafficPadding {
    pub fn pad(data: &[u8], target_size: usize) -> Vec<u8> {
        let mut padded = data.to_vec();

        if padded.len() < target_size {
            let pad_len = target_size - padded.len();
            let padding = SecureRng::random_bytes(pad_len);
            padded.extend(padding);
        }

        padded
    }

    pub fn random_pad(data: &[u8]) -> Vec<u8> {
        let extra = (SecureRng::random_u32() % 64) as usize;
        let mut padded = data.to_vec();
        padded.extend(SecureRng::random_bytes(extra));
        padded
    }
}
