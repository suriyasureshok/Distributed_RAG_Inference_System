//! # LLM Service
//!
//! Orchestrate resilient LLM generation with fallback behavior.
//!
//! ## Responsibilities
//! - Execute primary provider calls through resilience guards.
//! - Retry transient failures and enforce operation timeout.
//! - Use fallback provider when primary execution fails.

use std::sync::Arc;

use crate::providers::{fallback::FallbackProvider, primary::PrimaryProvider};
use shared::{
    domain::errors::AppError,
    resilience::{
        circuit_breaker::CircuitBreaker, retry::retry_with_backoff, timeout::with_timeout,
    },
};

/// Provide resilient LLM generation orchestration.
///
/// ## Fields
/// - `primary`: Primary generation provider.
/// - `fallback`: Fallback generation provider.
/// - `circuit_breaker`: Shared breaker guarding primary provider calls.
pub struct LLMService {
    /// Primary generation provider.
    primary: PrimaryProvider,
    /// Fallback generation provider.
    fallback: FallbackProvider,
    /// Shared breaker guarding primary provider calls.
    circuit_breaker: Arc<CircuitBreaker>,
}

impl LLMService {
    /// Create a new LLM service.
    ///
    /// ## Arguments
    /// - `circuit_breaker`: Shared breaker used to gate primary provider calls.
    ///
    /// ## Returns
    /// A configured `LLMService`.
    pub fn new(circuit_breaker: Arc<CircuitBreaker>) -> Self {
        Self {
            primary: PrimaryProvider,
            fallback: FallbackProvider,
            circuit_breaker,
        }
    }

    /// Generate an answer using primary provider with fallback behavior.
    ///
    /// ## Arguments
    /// - `context`: Prompt or context segments for generation.
    ///
    /// ## Returns
    /// Generated answer text from primary or fallback provider.
    ///
    /// ## Errors
    /// Returns error text when both primary and fallback generation fail.
    ///
    /// ## Performance
    /// Applies up to 3 retries with timeout and circuit-breaker checks.
    pub async fn generate(&self, context: Vec<String>) -> Result<String, String> {
        let cb = self.circuit_breaker.clone();
        let fallback_context = context.clone();

        let result = retry_with_backoff(3, || async {
            with_timeout(std::time::Duration::from_secs(2), async {
                cb.call(|| async {
                    self.primary
                        .generate(context.clone())
                        .await
                        .map_err(AppError::LLMError)
                })
                .await
            })
            .await
        })
        .await;

        match result {
            Ok(res) => Ok(res),
            Err(_) => self.fallback.generate(fallback_context).await,
        }
    }
}
