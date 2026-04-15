use crate::network::transport::stream::Stream;

pub struct QuicTransport;

impl QuicTransport {
    pub fn connect(addr: &str) -> Stream {
        println!("[QUIC] Establishing multiplexed connection to {}", addr);
        Stream::new()
    }

    pub fn stream_open(id: u32) {
        println!("[QUIC] Opening stream {}", id);
    }
}
