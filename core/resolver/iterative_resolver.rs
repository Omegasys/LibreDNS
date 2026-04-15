use crate::core::resolver::context::QueryContext;

pub struct IterativeResolver;

impl IterativeResolver {
    pub async fn resolve(ctx: &QueryContext) -> Option<String> {
        println!("Iterative resolving: {}", ctx.domain);

        // TODO:
        // - Query root nodes (DHT)
        // - Walk resolution chain
        // - Validate each step

        None
    }
}
