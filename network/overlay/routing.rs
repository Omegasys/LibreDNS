use crate::network::overlay::message::Message;

pub struct Router;

impl Router {
    pub fn route(message: &mut Message, next_hop: &str) {
        message.to = Some(next_hop.to_string());
        message.hop += 1;

        println!(
            "[Router] {} -> {} (hop {})",
            message.from,
            next_hop,
            message.hop
        );
    }

    pub fn deliver(message: &Message) {
        println!(
            "[Router] Delivering message {} type {:?}",
            message.id,
            message.msg_type
        );
    }
}
