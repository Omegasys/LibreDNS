mod doh_client;
mod dot_client;
mod doq_client;
mod encryption_layer;

use doh_client::DoHClient;
use dot_client::DoTClient;
use doq_client::DoQClient;

fn main() {
    println!("=== Encrypted Query Demo ===");

    let query = "example.ddns";

    let doh = DoHClient::new();
    let dot = DoTClient::new();
    let doq = DoQClient::new();

    println!("\n[DoH]");
    doh.query(query);

    println!("\n[DoT]");
    dot.query(query);

    println!("\n[DoQ]");
    doq.query(query);
}
