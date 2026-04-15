mod dhcp_client;
mod dhcp_node;
mod ip_allocator;
mod lease_demo;
mod simulation;

use simulation::run_simulation;

fn main() {
    println!("=== DHCP Network Demo (Decentralized Model) ===");

    run_simulation();
}
