use crate::dhcp_client::DhcpClient;
use crate::dhcp_node::DhcpNode;

pub fn run_simulation() {
    let mut node = DhcpNode::new("dhcp-node-1", 10);

    let clients = vec![
        DhcpClient::new("alice"),
        DhcpClient::new("bob"),
        DhcpClient::new("carol"),
        DhcpClient::new("dave"),
    ];

    for client in &clients {
        client.request_ip(&mut node);
    }

    println!("\n--- Lease Validation ---");
    node.validate_leases();
}
