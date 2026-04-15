use crate::core::dhcp::{
    discovery::{Discover, Offer},
    negotiation::{Request, Ack},
};

pub struct DhcpClient {
    pub id: String,
}

impl DhcpClient {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn create_discover(&self) -> Discover {
        Discover {
            client_id: self.id.clone(),
            public_key: vec![],
            nonce: vec![1, 2, 3], // TODO: secure nonce
        }
    }

    pub fn select_offer(&self, offers: Vec<Offer>) -> Option<Offer> {
        offers.into_iter().next()
    }

    pub fn create_request(&self, offer: &Offer) -> Request {
        Request {
            client_id: self.id.clone(),
            requested_ip: offer.ip.clone(),
            server_id: offer.server_id.clone(),
            signature: Vec::new(),
        }
    }

    pub fn handle_ack(&self, ack: Ack) {
        println!("Lease acquired: {} (expires {})", ack.ip, ack.expiration);
    }
}
