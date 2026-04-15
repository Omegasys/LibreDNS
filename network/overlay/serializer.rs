use crate::network::overlay::message::Message;

pub struct Serializer;

impl Serializer {
    pub fn encode(msg: &Message) -> Vec<u8> {
        format!("{:?}", msg).into_bytes()
    }

    pub fn decode(data: &[u8]) -> String {
        String::from_utf8_lossy(data).to_string()
    }
}
