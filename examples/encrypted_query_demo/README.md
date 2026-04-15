# Encrypted Query Demo (DoH / DoT / DoQ)

This example simulates encrypted DNS queries using three protocols:

- DNS-over-HTTPS (DoH)
- DNS-over-TLS (DoT)
- DNS-over-QUIC (DoQ)

## Features

- Shared encryption layer (XOR + SHA-256 key derivation)
- Simulated secure transport layers
- Mock DNS resolution
- Protocol separation architecture

## Run

```bash
cargo run --example encrypted_query_demo
