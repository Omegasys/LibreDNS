# LibreDNS Cryptography

## Overview

LibreDNS relies on modern cryptographic primitives.

---

## Algorithms

### Signing
- Ed25519 (preferred)

### Encryption
- AES-GCM (symmetric)
- ChaCha20-Poly1305 (alternative)

### Key Exchange
- X25519

### Hashing
- SHA-256 / BLAKE3

---

## Key Types

- Node identity key
- Domain ownership key
- Session keys

---

## DNS Record Signing

- Each record signed by owner
- Public key stored in DHT

---

## DHCP Lease Signing

- Lease signed by issuing node
- Verified by clients and peers

---

## Forward Secrecy

- Ephemeral session keys
- Key rotation supported

---

## Replay Protection

- Nonces
- Timestamps

---

## Trust Model

- No central CA
- Trust based on cryptographic proof
