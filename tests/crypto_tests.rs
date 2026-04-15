use ed25519_dalek::{Keypair, Signer, Verifier};
use rand::rngs::OsRng;

#[test]
fn test_signature_verification() {
    let mut csprng = OsRng {};
    let keypair = Keypair::generate(&mut csprng);

    let message = b"secure message";
    let signature = keypair.sign(message);

    assert!(keypair.verify(message, &signature).is_ok());
}

#[test]
fn test_signature_tampering_fails() {
    let mut csprng = OsRng {};
    let keypair = Keypair::generate(&mut csprng);

    let message = b"original";
    let signature = keypair.sign(message);

    let tampered = b"modified";

    assert!(keypair.verify(tampered, &signature).is_err());
}
