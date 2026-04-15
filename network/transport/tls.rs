use crate::network::transport::stream::Stream;

pub struct TlsWrapper;

impl TlsWrapper {
    pub fn wrap(stream: Stream) -> Stream {
        println!("[TLS] Wrapping stream with encryption");
        stream
    }

    pub fn handshake(addr: &str) {
        println!("[TLS] Handshake with {}", addr);
    }
}
