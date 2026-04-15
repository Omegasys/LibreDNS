use crate::node_config::NodeConfig;
use std::thread;
use std::time::Duration;

pub fn bootstrap_network(configs: Vec<NodeConfig>) {
    println!("Bootstrapping {} nodes...", configs.len());

    for config in configs {
        thread::spawn(move || {
            println!(
                "[{}] Starting at {}",
                config.node_id, config.listen_addr
            );

            for i in 0..3 {
                println!("[{}] heartbeat {}", config.node_id, i);
                thread::sleep(Duration::from_millis(500));
            }

            println!("[{}] shutting down (demo)", config.node_id);
        });
    }

    // Allow threads to run
    thread::sleep(Duration::from_secs(3));
}
