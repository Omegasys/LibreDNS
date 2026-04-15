use async_trait::async_trait;

#[async_trait]
pub trait Service: Send + Sync {
    async fn start(&self);
    async fn stop(&self);
}
