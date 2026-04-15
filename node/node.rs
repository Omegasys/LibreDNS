use crate::node::{
    config::NodeConfig,
    service::Service,
    metrics::Metrics,
    dhcp_service::DhcpService,
};

use crate::core::resolver::resolver::Resolver;
use crate::core::routing::p2p_router::P2PRouter;
use crate::core::dhcp::dhcp_server::DhcpServer;

use std::sync::{Arc, Mutex};

pub struct Node {
    config: NodeConfig,
    resolver: Arc<Mutex<Resolver>>,
    router: Arc<P2PRouter>,
    metrics: Metrics,
    services: Vec<Arc<dyn Service>>,
}

impl Node {
    pub fn new(config: NodeConfig) -> Self {
        let metrics = Metrics::new();

        let resolver = Arc::new(Mutex::new(Resolver::new()));
        let router = Arc::new(P2PRouter::new());

        let mut services: Vec<Arc<dyn Service>> = Vec::new();

        if config.enable_dhcp {
            let dhcp_server = DhcpServer::new(
                config.node_id.clone(),
                vec![
                    "192.168.1.10".to_string(),
                    "192.168.1.11".to_string(),
                ],
            );

            let dhcp_service = DhcpService::new(dhcp_server, metrics.clone());
            services.push(Arc::new(dhcp_service));
        }

        Self {
            config,
            resolver,
            router,
            metrics,
            services,
        }
    }

    pub async fn start(&self) {
        println!("Starting node: {}", self.config.node_id);

        for service in &self.services {
            service.start().await;
        }

        println!("Node started on {}", self.config.bind_address);
    }

    pub async fn shutdown(&self) {
        println!("Shutting down node...");

        for service in &self.services {
            service.stop().await;
        }

        println!("Node stopped");
    }

    pub fn metrics(&self) -> crate::node::metrics::MetricsData {
        self.metrics.snapshot()
    }
}
