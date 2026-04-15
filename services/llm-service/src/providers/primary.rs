//! # Primary Provider
//!
//! Implement primary LLM provider behavior.
//!
//! ## Responsibilities
//! - Simulate primary generation latency.
//! - Return provider errors for retry and fallback handling.

use std::time::Duration;

/// Represent the primary LLM provider.
pub struct PrimaryProvider;

impl PrimaryProvider {
    /// Generate text using the primary provider.
    ///
    /// ## Arguments
    /// - `context`: Prompt or context segments for generation.
    ///
    /// ## Returns
    /// Generated answer text when the provider succeeds.
    ///
    /// ## Errors
    /// Returns an error string when the simulated provider fails.
    ///
    /// ## Performance
    /// Adds a fixed artificial latency of 200 milliseconds.
    pub async fn generate(&self, context: Vec<String>) -> Result<String, String> {
        // simulate latency
        tokio::time::sleep(Duration::from_millis(200)).await;

        // simulate occasional failure
        if context.len() % 2 == 0 {
            return Err("Primary LLM failed".to_string());
        }

        Ok(format!("Primary response for {:?}", context))
    }
}
