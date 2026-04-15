use crate::network::transport::stream::Stream;

pub struct TcpTransport;

impl TcpTransport {
    pub fn connect(addr: &str) -> Stream {
        println!("[TCP] Connecting to {}", addr);
        Stream::new()
    }

    pub fn listen(addr: &str) {
        println!("[TCP] Listening on {}", addr);
    }
}
