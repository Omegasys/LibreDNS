use crate::core::dhcp::dhcp_server::DhcpServer;
use crate::node::service::Service;
use crate::node::metrics::Metrics;

use async_trait::async_trait;
use std::sync::{Arc, Mutex};

pub struct DhcpService {
    server: Arc<Mutex<DhcpServer>>,
    metrics: Metrics,
}

impl DhcpService {
    pub fn new(server: DhcpServer, metrics: Metrics) -> Self {
        Self {
            server: Arc::new(Mutex::new(server)),
            metrics,
        }
    }
}

#[async_trait]
impl Service for DhcpService {
    async fn start(&self) {
        println!("DHCP Service started");

        // Simulated loop
        let mut server = self.server.lock().unwrap();

        // Example: simulate one lease
        if let Some(offer) = server.handle_discover(
            crate::core::dhcp::discovery::Discover {
                client_id: "client-1".to_string(),
                public_key: vec![],
                nonce: vec![],
            }
        ) {
            println!("Offered IP: {}", offer.ip);
            self.metrics.inc_dhcp_leases();
        }
    }

    async fn stop(&self) {
        println!("DHCP Service stopped");
    }
}
