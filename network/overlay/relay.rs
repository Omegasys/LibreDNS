use crate::network::overlay::message::Message;

pub struct RelayNode {
    pub id: String,
}

impl RelayNode {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
        }
    }

    pub fn forward(&self, msg: &Message, next: &str) {
        println!(
            "[Relay {}] forwarding {} -> {}",
            self.id, msg.from, next
        );
    }
}
