#[derive(Debug, Clone)]
pub enum Message {
    Query {
        domain: String,
        record_type: u8,
    },
    Response {
        domain: String,
        value: String,
        ttl: u32,
    },
    Error {
        code: u16,
        message: String,
    },
}
