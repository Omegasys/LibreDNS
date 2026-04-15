use crate::core::dhcp::{
    ip_pool::IpPool,
    lease::Lease,
    discovery::{Discover, Offer},
    negotiation::{Request, Ack},
};

pub struct DhcpServer {
    pub id: String,
    pub pool: IpPool,
}

impl DhcpServer {
    pub fn new(id: String, ips: Vec<String>) -> Self {
        Self {
            id,
            pool: IpPool::new(ips),
        }
    }

    pub fn handle_discover(&mut self, msg: Discover) -> Option<Offer> {
        let ip = self.pool.allocate()?;

        Some(Offer {
            ip,
            lease_time: 3600,
            server_id: self.id.clone(),
            signature: Vec::new(), // TODO: sign
        })
    }

    pub fn handle_request(&mut self, req: Request) -> Option<Ack> {
        let lease = Lease::new(
            req.requested_ip.clone(),
            req.client_id.clone(),
            self.id.clone(),
            3600,
        );

        Some(Ack {
            lease_id: format!("{}-{}", req.client_id, req.requested_ip),
            ip: req.requested_ip,
            expiration: 3600,
            signature: Vec::new(), // TODO: sign
        })
    }
}
