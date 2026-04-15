use crate::client::resolver_client::ResolverClient;
use crate::client::dhcp_client::DhcpClientApp;
use std::env;

pub struct CLI;

impl CLI {
    pub async fn run() {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            println!("Usage:");
            println!("  resolve <domain>");
            println!("  dhcp");
            return;
        }

        match args[1].as_str() {
            "resolve" => {
                if args.len() < 3 {
                    println!("Usage: resolve <domain>");
                    return;
                }

                let domain = &args[2];
                let result = ResolverClient::resolve(domain).await;

                println!("Result: {:?}", result);
            }

            "dhcp" => {
                let client = DhcpClientApp::new("client-1".to_string());
                client.run();
            }

            _ => {
                println!("Unknown command");
            }
        }
    }
}
