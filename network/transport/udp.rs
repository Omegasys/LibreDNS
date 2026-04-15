use crate::network::transport::stream::Stream;

pub struct UdpTransport;

impl UdpTransport {
    pub fn send(addr: &str, data: &[u8]) {
        println!("[UDP] Sending {} bytes to {}", data.len(), addr);
    }

    pub fn receive() -> Stream {
        println!("[UDP] Receiving packet...");
        Stream::new()
    }
}
