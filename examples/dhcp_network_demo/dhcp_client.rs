use crate::dhcp_node::DhcpNode;

pub struct DhcpClient {
    pub id: String,
    pub pubkey: String,
}

impl DhcpClient {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            pubkey: format!("pubkey-{}", id),
        }
    }

    pub fn request_ip(&self, node: &mut DhcpNode) {
        println!("[CLIENT {}] Requesting IP...", self.id);

        let lease = node.assign_ip(&self.pubkey);

        match lease {
            Some(l) => {
                println!(
                    "[CLIENT {}] Received IP: {} (ttl: {})",
                    self.id, l.ip, l.ttl
                );
            }
            None => {
                println!("[CLIENT {}] IP request denied", self.id);
            }
        }
    }
}
