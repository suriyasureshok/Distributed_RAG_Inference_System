use std::time::Duration;
use tokio::time;

use crate::domain::errors::AppError;

pub async fn with_timeout<F, T>(
    duration: Duration,
    fut: F,
) -> Result<T, AppError>
where
    F: std::future::Future<Output = Result<T, AppError>>,
{
    match time::timeout(duration, fut).await {
        Ok(result) => result,
        Err(_) => Err(AppError::Timeout),
    }
}