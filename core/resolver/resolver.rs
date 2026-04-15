use crate::core::resolver::{
    context::{QueryContext, RecordType},
    recursive_resolver::RecursiveResolver,
    forwarder::Forwarder,
};

pub struct Resolver {
    recursive: RecursiveResolver,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            recursive: RecursiveResolver::new(),
        }
    }

    pub async fn resolve(&mut self, domain: &str, record_type: RecordType) -> Option<String> {
        let ctx = QueryContext::new(domain.to_string(), record_type);

        // Try recursive resolver
        if let Some(result) = self.recursive.resolve(ctx.clone()).await {
            return Some(result);
        }

        // Fallback
        Forwarder::forward_query(&ctx.domain).await
    }
}
