# LibreDNS API Specification

## Overview

Defines interfaces for interacting with LibreDNS.

---

## DNS API

### Resolve Domain
GET /resolve?domain=example.com&type=A


Response:

{
"domain": "example.com",
"ip": "1.2.3.4",
"ttl": 300
}


---

## Submit Record


POST /record


Body:

{
"domain": "example.com",
"type": "A",
"value": "1.2.3.4",
"signature": "..."
}


---

## DHCP API

### Request IP


POST /dhcp/request


---

### Release IP


POST /dhcp/release


---

## Node API

### Get Peers


GET /peers


---

### Node Status


GET /status


---

## Config API

### Update Settings


POST /config


---

## Security

- All endpoints require authentication
- Requests must be signed
- TLS required for HTTP interface

---

## Future Extensions

- WebSocket streaming
- GraphQL interface
- Plugin system
