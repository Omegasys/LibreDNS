pub struct Onion;

impl Onion {
    pub fn wrap(data: &[u8], hops: usize) -> Vec<u8> {
        let mut layered = data.to_vec();

        for _ in 0..hops {
            layered = Self::encrypt_layer(&layered);
        }

        layered
    }

    fn encrypt_layer(data: &[u8]) -> Vec<u8> {
        data.iter().map(|b| b ^ 0xAA).collect()
    }

    pub fn unwrap(data: &[u8], hops: usize) -> Vec<u8> {
        let mut result = data.to_vec();

        for _ in 0..hops {
            result = Self::decrypt_layer(&result);
        }

        result
    }

    fn decrypt_layer(data: &[u8]) -> Vec<u8> {
        data.iter().map(|b| b ^ 0xAA).collect()
    }
}
