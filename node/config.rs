#[derive(Clone)]
pub struct NodeConfig {
    pub node_id: String,
    pub bind_address: String,
    pub enable_dhcp: bool,
    pub enable_metrics: bool,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            node_id: "node-1".to_string(),
            bind_address: "0.0.0.0:5353".to_string(),
            enable_dhcp: true,
            enable_metrics: true,
        }
    }
}
