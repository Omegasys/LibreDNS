use crate::filtering::{
    allowlist::Allowlist,
    blocklist::Blocklist,
    matcher::Matcher,
    policy::Policy,
    stats::FilterStats,
};

pub struct FilterEngine {
    pub allowlist: Allowlist,
    pub blocklist: Blocklist,
    pub matcher: Matcher,
    pub policy: Policy,
    pub stats: FilterStats,
}

impl FilterEngine {
    pub fn new(policy: Policy) -> Self {
        Self {
            allowlist: Allowlist::new(),
            blocklist: Blocklist::new(),
            matcher: Matcher::new(),
            policy,
            stats: FilterStats::new(),
        }
    }

    pub fn evaluate(&mut self, domain: &str) -> bool {
        self.stats.total_queries += 1;

        if self.allowlist.contains(domain) {
            self.stats.allowed += 1;
            return true;
        }

        if self.blocklist.contains(domain) {
            self.stats.blocked += 1;
            return false;
        }

        let category = self.matcher.classify(domain);

        if self.policy.block_category(&category) {
            self.stats.blocked += 1;
            return false;
        }

        self.stats.allowed += 1;
        true
    }
}
