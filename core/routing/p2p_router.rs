use crate::core::routing::{
    routing_table::RoutingTable,
    latency::LatencyTracker,
    path_builder::PathBuilder,
    failover::Failover,
};

use std::time::Duration;

pub struct P2PRouter {
    pub routing_table: RoutingTable,
    pub latency: LatencyTracker,
}

impl P2PRouter {
    pub fn new() -> Self {
        Self {
            routing_table: RoutingTable::new(),
            latency: LatencyTracker::new(),
        }
    }

    pub async fn route_query(
        &self,
        payload: Vec<u8>,
    ) -> Option<Vec<u8>> {
        let path = PathBuilder::build_path(&self.routing_table, &self.latency, 3);

        println!("Routing through path: {:?}", path);

        // Simulated send with failover
        Failover::retry(
            || async {
                // TODO: replace with real network send
                println!("Sending payload...");
                Some(payload.clone())
            },
            3,
            Duration::from_millis(200),
        )
        .await
    }
}
