# LibreDNS Protocol Specification

## Overview

LibreDNS defines a custom DNS protocol designed for:
- Decentralization
- Encryption
- Integrity verification

---

## Packet Structure

### Header

| Field            | Size | Description                  |
|------------------|------|------------------------------|
| Version          | 1B   | Protocol version             |
| Message Type     | 1B   | Query / Response / Update    |
| Flags            | 2B   | Control flags                |
| Transaction ID   | 4B   | Unique request ID            |

---

### Body

#### Query
- Domain name
- Record type (A, AAAA, TXT, etc.)

#### Response
- Answer records
- TTL
- Signature

---

## Message Types

- QUERY
- RESPONSE
- UPDATE
- ERROR

---

## Record Format
{
domain: string,
type: enum,
value: bytes,
ttl: u32,
signature: bytes
}

---

## Validation

- Records must be signed
- Signature verified using public key
- Chain of trust validated

---

## Transport

Supported transports:
- UDP (fallback)
- TCP
- QUIC (preferred)
- HTTPS (DoH compatibility)

---

## Routing

- Direct P2P
- Onion routing (optional)
- Multi-hop forwarding

---

## Error Handling

- INVALID_SIGNATURE
- RECORD_NOT_FOUND
- TIMEOUT
- MALFORMED_PACKET
