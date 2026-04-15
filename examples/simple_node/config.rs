#[derive(Debug, Clone)]
pub struct Config {
    pub node_id: String,
    pub listen_addr: String,
    pub bootstrap_nodes: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            node_id: "node-1".to_string(),
            listen_addr: "127.0.0.1:8080".to_string(),
            bootstrap_nodes: vec![],
        }
    }
}
