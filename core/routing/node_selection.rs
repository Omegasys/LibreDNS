use crate::core::routing::{routing_table::RoutingTable, latency::LatencyTracker};

pub struct NodeSelector;

impl NodeSelector {
    pub fn select_best_nodes(
        routing_table: &RoutingTable,
        latency: &LatencyTracker,
        count: usize,
    ) -> Vec<String> {
        let mut candidates = routing_table.all_peers();

        candidates.sort_by_key(|peer| {
            latency.get(&peer.id).unwrap_or_default()
        });

        candidates
            .into_iter()
            .take(count)
            .map(|p| p.id)
            .collect()
    }
}
