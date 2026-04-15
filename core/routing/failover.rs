use std::time::Duration;

pub struct Failover;

impl Failover {
    pub async fn retry<F, Fut, T>(
        mut operation: F,
        retries: u8,
        delay: Duration,
    ) -> Option<T>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Option<T>>,
    {
        for _ in 0..retries {
            if let Some(result) = operation().await {
                return Some(result);
            }
            tokio::time::sleep(delay).await;
        }
        None
    }
}
