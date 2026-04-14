//! # Client Config
//!
//! Define shared timeout and retry settings for downstream clients.

use std::time::Duration;

#[derive(Clone)]
/// Represent per-client resilience settings.
///
/// ## Fields
/// - `timeout`: Operation deadline.
/// - `max_retries`: Maximum retry attempts for retryable failures.
pub struct ClientConfig {
    /// Operation deadline.
    pub timeout: Duration,
    /// Maximum retry attempts for retryable failures.
    pub max_retries: u32,
}
