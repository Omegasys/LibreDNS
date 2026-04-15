use rand::rngs::OsRng;
use rand::RngCore;

pub struct SecureRng;

impl SecureRng {
    pub fn random_bytes(len: usize) -> Vec<u8> {
        let mut buf = vec![0u8; len];
        OsRng.fill_bytes(&mut buf);
        buf
    }

    pub fn random_u32() -> u32 {
        OsRng.next_u32()
    }

    pub fn random_u64() -> u64 {
        OsRng.next_u64()
    }
}
