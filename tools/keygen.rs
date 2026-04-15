use ed25519_dalek::{Keypair, Signer};
use rand::rngs::OsRng;
use base64::{engine::general_purpose, Engine as _};

pub fn generate_keypair() -> Keypair {
    let mut csprng = OsRng {};
    Keypair::generate(&mut csprng)
}

pub fn print_keypair() {
    let keypair = generate_keypair();

    let public = general_purpose::STANDARD.encode(keypair.public.to_bytes());
    let private = general_purpose::STANDARD.encode(keypair.secret.to_bytes());

    println!("Public Key: {}", public);
    println!("Private Key: {}", private);
}

fn main() {
    print_keypair();
}
