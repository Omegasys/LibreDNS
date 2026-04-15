use crate::network::peer::peer::Peer;

pub struct Reputation;

impl Reputation {
    pub fn score_peer(peer: &Peer) -> f32 {
        let mut score = peer.reputation;

        if peer.connected {
            score += 0.2;
        }

        if score < 0.0 {
            return 0.0;
        }

        if score > 10.0 {
            return 10.0;
        }

        score
    }

    pub fn is_trusted(peer: &Peer) -> bool {
        Self::score_peer(peer) > 0.5
    }
}
