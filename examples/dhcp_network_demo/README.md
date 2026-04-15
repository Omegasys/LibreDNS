# DHCP Network Demo (Decentralized Design)

This example simulates a **decentralized DHCP system** with deterministic IP allocation.

---

## Features

### Core System
- No central DHCP server (simulated node authority)
- Deterministic IP allocation using SHA-256
- Lease tracking per public key
- TTL-based lease expiration

### Simulation
- Multiple clients request IPs
- Single DHCP node assigns addresses
- Lease validation system
- Conflict-free allocation model

---

## How It Works

### 1. Client Requests IP
Each client has a public key.

### 2. Deterministic Allocation
IP = HASH(pubkey) → subnet + host byte


### 3. Lease Creation
- Timestamped lease
- TTL expiration
- Stored per node

---

## Run

```bash
cargo run --example dhcp_network_demo
