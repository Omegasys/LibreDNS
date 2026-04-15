use crate::core::resolver::{
    cache::ResolverCache,
    context::QueryContext,
    iterative_resolver::IterativeResolver,
    validator::Validator,
};

pub struct RecursiveResolver {
    pub cache: ResolverCache,
}

impl RecursiveResolver {
    pub fn new() -> Self {
        Self {
            cache: ResolverCache::new(),
        }
    }

    pub async fn resolve(&mut self, ctx: QueryContext) -> Option<String> {
        let cache_key = format!("{}:{:?}", ctx.domain, ctx.record_type);

        // 1. Check cache
        if let Some(result) = self.cache.get(&cache_key) {
            println!("Cache hit: {}", ctx.domain);
            return Some(result);
        }

        // 2. Iterative resolution
        if let Some(result) = IterativeResolver::resolve(&ctx).await {
            // 3. Validate
            if Validator::validate_record(&ctx.domain, &result, &[]) {
                // 4. Cache result
                self.cache.insert(cache_key, result.clone(), 300);
                return Some(result);
            }
        }

        None
    }
}
