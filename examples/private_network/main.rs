mod bootstrap;
mod topology;
mod node_config;

use bootstrap::bootstrap_network;
use topology::NetworkTopology;
use node_config::NodeConfig;

fn main() {
    println!("Starting private network...");

    let topology = NetworkTopology::new_mesh(5);
    let configs = topology.generate_configs();

    bootstrap_network(configs);
}
