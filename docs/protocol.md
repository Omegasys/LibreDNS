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
