use crate::core::protocol::{
    packet::Packet,
    header::Header,
    message::Message,
    serializer::Serializer,
};

pub struct ResolverClient;

impl ResolverClient {
    pub async fn resolve(domain: &str) -> Option<Vec<u8>> {
        let header = Header::new(1, 0, 1234);

        let message = Message::Query {
            domain: domain.to_string(),
            record_type: 1,
        };

        let packet = Packet::new(header, message);

        let serialized = Serializer::serialize(&packet).ok()?;

        // TODO: send over network
        println!("Sending query: {:?}", serialized);

        Some(serialized)
    }
}
