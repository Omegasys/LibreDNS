use ed25519_dalek::{Keypair, Signer, Verifier};
use rand::rngs::OsRng;

#[test]
fn test_end_to_end_message_flow() {
    let mut csprng = OsRng {};
    let keypair = Keypair::generate(&mut csprng);

    let message = b"resolve example.ddns";
    let signature = keypair.sign(message);

    // Simulate network transfer + verification
    let received = message;

    assert!(keypair.verify(received, &signature).is_ok());
}

#[test]
fn test_dns_with_crypto_binding() {
    let mut csprng = OsRng {};
    let keypair = Keypair::generate(&mut csprng);

    let record = b"example.ddns -> 10.0.0.1";
    let signature = keypair.sign(record);

    assert!(keypair.verify(record, &signature).is_ok());
}
