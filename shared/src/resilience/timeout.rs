//! # Timeout
//!
//! Provide timeout wrappers for asynchronous operations.

use std::time::Duration;
use tokio::time;

use crate::domain::errors::AppError;

/// Execute a future and fail if it does not complete within the timeout.
///
/// ## Type Parameters
/// - `F`: Future type.
/// - `T`: Successful result type.
///
/// ## Arguments
/// - `duration`: Maximum allowed execution time.
/// - `fut`: Future to execute.
///
/// ## Returns
/// The future result when it completes within the deadline.
///
/// ## Errors
/// Returns `AppError::Timeout` when the deadline is exceeded.
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