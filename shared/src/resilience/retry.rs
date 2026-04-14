//! # Retry
//!
//! Provide retry logic with exponential backoff for retryable failures.

use std::time::Duration;
use tokio::time::sleep;
use crate::domain::errors::AppError;

/// Execute an asynchronous task with retry and exponential backoff.
///
/// ## Type Parameters
/// - `F`: Closure that creates a future for each attempt.
/// - `Fut`: Future returned by `F`.
/// - `T`: Successful result type.
///
/// ## Arguments
/// - `max_retries`: Maximum number of retry attempts after the first failure.
/// - `task`: Asynchronous task to execute.
///
/// ## Returns
/// The first successful task result.
///
/// ## Errors
/// Returns the last encountered non-retryable error, or the final error after
/// exhausting retries.
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