use crate::node_config::NodeConfig;

pub struct NetworkTopology {
    size: usize,
}

impl NetworkTopology {
    pub fn new_mesh(size: usize) -> Self {
        Self { size }
    }

    pub fn generate_configs(&self) -> Vec<NodeConfig> {
        let mut configs = Vec::new();

        for i in 0..self.size {
            let node_id = format!("node-{}", i);
            let listen_addr = format!("127.0.0.1:{}", 9000 + i);

            let peers = (0..self.size)
                .filter(|&j| j != i)
                .map(|j| format!("127.0.0.1:{}", 9000 + j))
                .collect();

            configs.push(NodeConfig {
                node_id,
                listen_addr,
                peers,
            });
        }

        configs
    }
}
