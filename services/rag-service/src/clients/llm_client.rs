//! # LLM Client
//!
//! Provide response generation with retry, timeout, and circuit-breaker guards.

use std::sync::Arc;

use shared::domain::errors::AppError;
use shared::observability::logger::{log_error, log_info};
use shared::resilience::circuit_breaker::CircuitBreaker;
use shared::resilience::retry::retry_with_backoff;
use shared::resilience::timeout::with_timeout;

use crate::config::client_config::ClientConfig;

/// Wrap LLM generation calls behind resilience policies and fallback behavior.
pub struct LLMClient {
    config: ClientConfig,
    circuit_breaker: Arc<CircuitBreaker>,
}

impl LLMClient {
    /// Create a new LLM client.
    ///
    /// ## Arguments
    /// - `config`: Timeout and retry configuration.
    /// - `circuit_breaker`: Shared breaker used to gate downstream calls.
    ///
    /// ## Returns
    /// A configured `LLMClient`.
    pub fn new(config: ClientConfig, circuit_breaker: Arc<CircuitBreaker>) -> Self {
        Self {
            config,
            circuit_breaker,
        }
    }

    /// Generate a response from retrieved context documents.
    ///
    /// ## Arguments
    /// - `_context`: Retrieved context snippets.
    ///
    /// ## Returns
    /// Generated response text.
    ///
    /// ## Errors
    /// Returns `AppError` for non-retryable or terminal failures.
    pub async fn generate(&self, _context: Vec<String>) -> Result<String, AppError> {
        let config = &self.config;
        let cb = self.circuit_breaker.clone();
        let fallback_context = _context.clone();

        log_info("Calling LLM");

        let result = retry_with_backoff(config.max_retries, || async {
            with_timeout(config.timeout, async {
                cb.call(|| async { Ok("Generated response".to_string()) })
                    .await
            })
            .await
        })
        .await;

        match result {
            Ok(res) => Ok(res),
            Err(err) => {
                if err.is_retryable() || matches!(err, AppError::Timeout | AppError::CircuitOpen) {
                    log_error("LLM failed, using fallback");
                    self.fallback(fallback_context).await
                } else {
                    Err(err)
                }
            }
        }
    }

    /// Return a fallback response when primary generation fails.
    ///
    /// ## Arguments
    /// - `_context`: Retrieved context snippets.
    ///
    /// ## Returns
    /// Fallback response text.
    async fn fallback(&self, _context: Vec<String>) -> Result<String, AppError> {
        // fallback logic (cheap model / default response)
        Ok("Fallback response".to_string())
    }
}
