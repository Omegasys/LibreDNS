use crate::network::overlay::message::Message;

pub struct Gossip;

impl Gossip {
    pub fn propagate(message: &Message, peers: &[String]) {
        println!(
            "[Gossip] Propagating message {} to {} peers",
            message.id,
            peers.len()
        );

        for peer in peers {
            println!("[Gossip] -> {}", peer);
        }
    }
}
