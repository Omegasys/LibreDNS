use crate::filtering::categories::Category;

#[derive(Clone)]
pub struct Rule {
    pub domain_pattern: String,
    pub category: Option<Category>,
    pub block: bool,
}

impl Rule {
    pub fn matches(&self, domain: &str) -> bool {
        if self.domain_pattern == "*" {
            return true;
        }

        domain.contains(&self.domain_pattern)
    }
}
