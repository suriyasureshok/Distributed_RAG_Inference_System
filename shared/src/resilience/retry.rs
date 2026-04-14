use std::time::Duration;
use tokio::time::sleep;
use crate::domain::errors::AppError;

pub async fn retry_with_backoff<F, Fut, T>(
    max_retries: u32,
    mut task: F,
) -> Result<T, AppError>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, AppError>>,
{
    let mut attempt: u32 = 0;

    loop {
        match task().await {
            Ok(val) => return Ok(val),
            Err(err) if attempt < max_retries && err.is_retryable() => {
                attempt += 1;

                let backoff = Duration::from_millis(100 * 2_u64.pow(attempt));
                sleep(backoff).await;
            }
            Err(err) => return Err(err),
        }
    }
}