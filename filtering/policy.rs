use crate::filtering::categories::Category;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Policy {
    blocked_categories: HashSet<Category>,
}

impl Policy {
    pub fn new() -> Self {
        Self {
            blocked_categories: HashSet::new(),
        }
    }

    pub fn block_category(&self, category: &Category) -> bool {
        self.blocked_categories.contains(category)
    }

    pub fn block(&mut self, category: Category) {
        self.blocked_categories.insert(category);
    }
}
