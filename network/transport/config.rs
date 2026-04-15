#[derive(Debug, Clone)]
pub enum TransportType {
    Tcp,
    Udp,
    Quic,
}

#[derive(Debug, Clone)]
pub struct TransportConfig {
    pub transport_type: TransportType,
    pub bind_addr: String,
    pub remote_addr: Option<String>,
    pub use_tls: bool,
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self {
            transport_type: TransportType::Tcp,
            bind_addr: "127.0.0.1:0".to_string(),
            remote_addr: None,
            use_tls: false,
        }
    }
}
