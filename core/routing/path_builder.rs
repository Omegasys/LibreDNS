use crate::core::routing::node_selection::NodeSelector;
use crate::core::routing::routing_table::RoutingTable;
use crate::core::routing::latency::LatencyTracker;

pub struct PathBuilder;

impl PathBuilder {
    pub fn build_path(
        routing_table: &RoutingTable,
        latency: &LatencyTracker,
        hops: usize,
    ) -> Vec<String> {
        let mut path = NodeSelector::select_best_nodes(routing_table, latency, hops);

        // Shuffle or randomize for privacy (future improvement)
        path
    }
}
