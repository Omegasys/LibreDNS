# LibreDNS Architecture

## Overview

LibreDNS is a decentralized, privacy-preserving network infrastructure that combines:

- Distributed DNS resolution
- Secure decentralized DHCP
- Encrypted multi-hop routing
- Peer-to-peer networking

The system eliminates centralized trust and provides censorship-resistant name resolution and network configuration.

---

## High-Level Architecture

### Layers

1. **Client Layer**
   - CLI / API interface
   - Sends DNS queries and DHCP requests

2. **Resolver Layer**
   - Recursive and iterative resolution
   - Cache management
   - Validation (DNSSEC-like)

3. **DHCP Layer**
   - Distributed IP allocation
   - Lease negotiation and validation

4. **Network Layer**
   - P2P communication
   - Transport protocols (UDP, TCP, QUIC)

5. **Overlay Layer**
   - DHT (Distributed Hash Table)
   - Gossip protocol
   - Optional consensus

6. **Security Layer**
   - Cryptographic verification
   - Anti-spoofing protections

7. **Privacy Layer**
   - Onion routing
   - Query anonymization
   - Traffic obfuscation

---

## Node Types

### Full Node
- Participates in DNS resolution
- Stores records in DHT
- Validates queries and leases

### Light Client
- Sends queries
- Does not store full state

### Relay Node
- Used for onion routing
- Forwards encrypted traffic

### DHCP Node
- Participates in IP allocation
- Signs and validates leases

---

## Data Flow

### DNS Query Flow
1. Client sends encrypted query
2. Routed via onion path (optional)
3. Resolver queries DHT
4. Record validated
5. Response returned

### DHCP Flow
1. Client broadcasts discovery
2. Multiple nodes propose IPs
3. Client selects offer
4. Lease signed and stored in DHT

---

## Storage

- Distributed via DHT
- Records include:
  - Domain → IP mappings
  - Lease assignments
  - Public keys

---

## Design Principles

- No central authority
- Cryptographic trust
- Privacy by default
- Modular architecture
- Transport-agnostic
