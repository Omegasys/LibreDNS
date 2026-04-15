use blake3;

pub struct Hasher;

impl Hasher {
    pub fn hash(data: &[u8]) -> Vec<u8> {
        blake3::hash(data).as_bytes().to_vec()
    }

    pub fn hash_hex(data: &[u8]) -> String {
        blake3::hash(data).to_hex().to_string()
    }
}
