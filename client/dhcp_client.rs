use crate::core::dhcp::{
    dhcp_client::DhcpClient,
    discovery::Offer,
};

pub struct DhcpClientApp {
    client: DhcpClient,
}

impl DhcpClientApp {
    pub fn new(id: String) -> Self {
        Self {
            client: DhcpClient::new(id),
        }
    }

    pub fn run(&self) {
        let discover = self.client.create_discover();
        println!("Sending DISCOVER: {:?}", discover);

        // Simulated offers
        let offers = vec![
            Offer {
                ip: "192.168.1.10".to_string(),
                lease_time: 3600,
                server_id: "node-1".to_string(),
                signature: vec![],
            }
        ];

        if let Some(offer) = self.client.select_offer(offers) {
            let request = self.client.create_request(&offer);
            println!("Sending REQUEST: {:?}", request);
        }
    }
}
