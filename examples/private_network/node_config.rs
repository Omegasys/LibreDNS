#[derive(Debug, Clone)]
pub struct NodeConfig {
    pub node_id: String,
    pub listen_addr: String,
    pub peers: Vec<String>,
}
