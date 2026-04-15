use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct NetworkEvent {
    pub timestamp: u128,
    pub source: String,
    pub destination: String,
    pub size: usize,
}

pub struct NetworkDebugger {
    events: Vec<NetworkEvent>,
    traffic_map: HashMap<String, usize>,
}

impl NetworkDebugger {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            traffic_map: HashMap::new(),
        }
    }

    pub fn log_event(&mut self, source: &str, destination: &str, size: usize) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let event = NetworkEvent {
            timestamp,
            source: source.to_string(),
            destination: destination.to_string(),
            size,
        };

        self.events.push(event);

        *self.traffic_map.entry(source.to_string()).or_insert(0) += size;
    }

    pub fn print_summary(&self) {
        println!("=== Network Summary ===");

        for (node, traffic) in &self.traffic_map {
            println!("Node: {} | Total Sent: {} bytes", node, traffic);
        }

        println!("\nTotal Events: {}", self.events.len());
    }

    pub fn print_events(&self) {
        println!("=== Network Events ===");

        for event in &self.events {
            println!(
                "[{}] {} -> {} ({} bytes)",
                event.timestamp, event.source, event.destination, event.size
            );
        }
    }
}
