mod config;
mod node_runner;

use config::Config;
use node_runner::NodeRunner;

fn main() {
    let config = Config::default();

    println!("Starting simple node...");
    let mut node = NodeRunner::new(config);

    node.start();
}
