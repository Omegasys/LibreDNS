# LibreDNS Threat Model

## Overview

This document outlines potential threats and defenses.

---

## Threats

### 1. DNS Spoofing
- Fake responses injected

**Mitigation:**
- Signature validation
- DNSSEC-like model

---

### 2. Cache Poisoning
- Malicious records stored

**Mitigation:**
- Signed records
- TTL enforcement

---

### 3. Rogue DHCP Nodes
- Assign fake IPs

**Mitigation:**
- Signed leases
- Node identity verification

---

### 4. Man-in-the-Middle (MITM)
- Intercepted traffic

**Mitigation:**
- End-to-end encryption
- Key exchange protocols

---

### 5. Traffic Correlation
- Tracking user activity

**Mitigation:**
- Onion routing
- Traffic padding

---

### 6. Sybil Attacks
- Fake nodes flooding network

**Mitigation:**
- Reputation system
- Stake or identity verification

---

### 7. DHT Poisoning
- Fake records injected

**Mitigation:**
- Record signatures
- Multi-node validation

---

## Assumptions

- Cryptography is secure
- Majority of nodes are honest

---

## Out of Scope

- Endpoint compromise
- Physical attacks
