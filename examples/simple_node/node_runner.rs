use crate::config::Config;
use std::{thread, time::Duration};

pub struct NodeRunner {
    config: Config,
    running: bool,
}

impl NodeRunner {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            running: false,
        }
    }

    pub fn start(&mut self) {
        self.running = true;

        println!("Node ID: {}", self.config.node_id);
        println!("Listening on: {}", self.config.listen_addr);

        self.event_loop();
    }

    fn event_loop(&self) {
        println!("Entering event loop...");

        for i in 0..5 {
            println!("Tick {}", i);
            thread::sleep(Duration::from_secs(1));
        }

        println!("Node shutting down (demo)");
    }
}
