pub struct Forwarder;

impl Forwarder {
    pub async fn forward_query(domain: &str) -> Option<String> {
        // Placeholder for fallback (e.g., legacy DNS or bootstrap nodes)
        println!("Forwarding query for {}", domain);

        // TODO: implement real network request
        None
    }
}
