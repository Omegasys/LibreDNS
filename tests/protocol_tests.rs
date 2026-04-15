use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct DnsMessage {
    name: String,
    record_type: String,
    value: String,
}

#[test]
fn test_dns_message_serialization() {
    let msg = DnsMessage {
        name: "example.ddns".into(),
        record_type: "A".into(),
        value: "192.168.1.1".into(),
    };

    let serialized = serde_json::to_string(&msg).unwrap();
    let deserialized: DnsMessage = serde_json::from_str(&serialized).unwrap();

    assert_eq!(msg, deserialized);
}

#[test]
fn test_dns_message_integrity() {
    let msg = DnsMessage {
        name: "secure.ddns".into(),
        record_type: "TXT".into(),
        value: "verified".into(),
    };

    assert!(!msg.name.is_empty());
    assert!(!msg.value.is_empty());
}
