# LibreDNS DHCP Protocol (D-DHCP)

## Overview

LibreDNS includes a decentralized DHCP system for secure IP allocation.

---

## Goals

- Eliminate centralized DHCP servers
- Prevent rogue DHCP attacks
- Enable cryptographically verifiable leases

---

## Message Flow

### 1. DISCOVER
Client broadcasts request:
{
client_id,
public_key,
nonce
}

---

### 2. OFFER
Nodes respond with:
{
offered_ip,
lease_time,
node_id,
signature
}


---

### 3. REQUEST
Client selects an offer:

{
selected_ip,
node_id,
client_signature
}


---

### 4. ACK
Node finalizes lease:

{
lease_id,
expiration,
signature
}


---

## Lease Structure


{
ip_address,
client_public_key,
lease_start,
lease_end,
issuer_node,
signature
}


---

## Conflict Resolution

- DHT tracks active leases
- Duplicate detection via hash
- Consensus or deterministic allocation

---

## Security Features

- Signed leases
- Replay protection (nonce)
- Node authentication
- Lease revocation

---

## Privacy Enhancements

- Optional anonymous leases
- Encrypted negotiation
