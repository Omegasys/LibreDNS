pub struct Topology {
    pub nodes: Vec<String>,
}

impl Topology {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    pub fn add_node(&mut self, node: &str) {
        self.nodes.push(node.to_string());
    }

    pub fn neighbors(&self, node: &str) -> Vec<String> {
        self.nodes
            .iter()
            .filter(|n| *n != node)
            .cloned()
            .collect()
    }
}
