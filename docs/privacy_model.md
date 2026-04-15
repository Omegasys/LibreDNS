# LibreDNS Privacy Model

## Overview

LibreDNS is designed to prevent tracking and surveillance.

---

## Goals

- Hide user identity
- Hide query content
- Prevent metadata leakage

---

## Techniques

### 1. Encryption
- All queries encrypted
- No plaintext DNS

---

### 2. Onion Routing
- Multi-hop relay system
- Each node knows only next hop

---

### 3. Query Obfuscation
- Randomized padding
- Dummy queries

---

### 4. Metadata Protection
- No IP leakage
- Encrypted headers (future)

---

### 5. No Logging
- Nodes do not store queries
- Optional local logs only

---

### 6. ODoH-like Separation
- Resolver does not know client
- Relay does not know query

---

## Threats Addressed

- ISP surveillance
- Government monitoring
- Corporate tracking

---

## Tradeoffs

- Increased latency
- Higher bandwidth usage
