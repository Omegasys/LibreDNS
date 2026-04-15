use crate::filtering::categories::Category;

pub struct Matcher;

impl Matcher {
    pub fn new() -> Self {
        Self
    }

    pub fn classify(&self, domain: &str) -> Category {
        if domain.contains("phish") {
            Category::Phishing
        } else if domain.contains("ads") {
            Category::Ads
        } else if domain.contains("malware") {
            Category::Malware
        } else {
            Category::Safe
        }
    }
}
