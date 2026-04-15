use rand::Rng;

#[derive(Clone)]
pub struct SimNode {
    pub id: String,
}

pub struct Simulator {
    pub nodes: Vec<SimNode>,
}

impl Simulator {
    pub fn new(count: usize) -> Self {
        let nodes = (0..count)
            .map(|i| SimNode {
                id: format!("node-{}", i),
            })
            .collect();

        Self { nodes }
    }

    pub fn random_node(&self) -> Option<&SimNode> {
        if self.nodes.is_empty() {
            return None;
        }

        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.nodes.len());
        self.nodes.get(index)
    }

    pub fn simulate_message(&self) {
        let from = self.random_node();
        let to = self.random_node();

        if let (Some(f), Some(t)) = (from, to) {
            println!("Simulated Message: {} -> {}", f.id, t.id);
        }
    }

    pub fn run(&self, iterations: usize) {
        for _ in 0..iterations {
            self.simulate_message();
        }
    }
}

fn main() {
    let sim = Simulator::new(10);
    sim.run(20);
}
