#[derive(Default)]
pub struct FilterStats {
    pub total_queries: usize,
    pub allowed: usize,
    pub blocked: usize,
}

impl FilterStats {
    pub fn print(&self) {
        println!("=== Filter Stats ===");
        println!("Total: {}", self.total_queries);
        println!("Allowed: {}", self.allowed);
        println!("Blocked: {}", self.blocked);
    }
}
