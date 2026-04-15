//! # LLM Service
//!
//! Orchestrate resilient LLM generation with fallback behavior.
//!
//! ## Responsibilities
//! - Execute primary provider calls through resilience guards.
//! - Retry transient failures and enforce operation timeout.
//! - Use fallback provider when primary execution fails.

use std::sync::Arc;
use std::time::Duration;

use crate::providers::{fallback::FallbackProvider, primary::PrimaryProvider};
use shared::{
    domain::errors::AppError,
    resilience::{
        circuit_breaker::CircuitBreaker, retry::retry_with_backoff, timeout::with_timeout,
    },
};

/// Represent configurable runtime settings for `LLMService`.
///
/// ## Fields
/// - `llm_timeout_secs`: Timeout for primary generation calls.
/// - `fallback_timeout_secs`: Timeout for fallback generation calls.
pub struct LLMServiceConfig {
    /// Timeout for primary generation calls.
    pub llm_timeout_secs: u64,
    /// Timeout for fallback generation calls.
    pub fallback_timeout_secs: u64,
}

impl Default for LLMServiceConfig {
    fn default() -> Self {
        Self {
            llm_timeout_secs: 30,
            fallback_timeout_secs: 30,
        }
    }
}

impl LLMServiceConfig {
    /// Load service configuration from environment variables.
    ///
    /// ## Returns
    /// An `LLMServiceConfig` using environment overrides or sensible defaults.
    ///
    /// ## Environment
    /// - `LLM_TIMEOUT_SECS`: Primary call timeout in seconds.
    /// - `LLM_FALLBACK_TIMEOUT_SECS`: Fallback call timeout in seconds.
    pub fn from_env() -> Self {
        Self {
            llm_timeout_secs: parse_timeout_secs("LLM_TIMEOUT_SECS", 30),
            fallback_timeout_secs: parse_timeout_secs("LLM_FALLBACK_TIMEOUT_SECS", 30),
        }
    }
}

/// Parse timeout seconds from an environment variable.
///
/// Returns the default value when parsing fails or value is zero.
fn parse_timeout_secs(var_name: &str, default_secs: u64) -> u64 {
    std::env::var(var_name)
        .ok()
        .and_then(|raw| raw.parse::<u64>().ok())
        .filter(|secs| *secs > 0)
        .unwrap_or(default_secs)
}

/// Provide resilient LLM generation orchestration.
///
/// ## Fields
/// - `primary`: Primary generation provider.
/// - `fallback`: Fallback generation provider.
/// - `circuit_breaker`: Shared breaker guarding primary provider calls.
/// - `config`: Runtime timeout settings for primary and fallback calls.
pub struct LLMService {
    /// Primary generation provider.
    primary: PrimaryProvider,
    /// Fallback generation provider.
    fallback: FallbackProvider,
    /// Shared breaker guarding primary provider calls.
    circuit_breaker: Arc<CircuitBreaker>,
    /// Runtime timeout settings for primary and fallback calls.
    config: LLMServiceConfig,
}

impl LLMService {
    /// Create a new LLM service.
    ///
    /// ## Arguments
    /// - `circuit_breaker`: Shared breaker used to gate primary provider calls.
    /// - `config`: Runtime timeout settings.
    ///
    /// ## Returns
    /// A configured `LLMService`.
    pub fn new(circuit_breaker: Arc<CircuitBreaker>, config: LLMServiceConfig) -> Self {
        Self {
            primary: PrimaryProvider,
            fallback: FallbackProvider,
            circuit_breaker,
            config,
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
    /// Returns error text when both primary and fallback generation fail or time out.
    ///
    /// ## Performance
    /// Applies up to 3 retries with timeout and circuit-breaker checks.
    pub async fn generate(&self, context: Vec<String>) -> Result<String, String> {
        let cb = self.circuit_breaker.clone();
        let fallback_context = context.clone();
        let primary_timeout = Duration::from_secs(self.config.llm_timeout_secs);
        let fallback_timeout = Duration::from_secs(self.config.fallback_timeout_secs);

        let result = retry_with_backoff(3, || async {
            with_timeout(primary_timeout, async {
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
            Err(e) => {
                tracing::warn!(error = ?e, "Primary provider failed, falling back");
                match tokio::time::timeout(
                    fallback_timeout,
                    self.fallback.generate(fallback_context),
                )
                .await
                {
                    Ok(fallback_result) => fallback_result,
                    Err(_) => Err(AppError::Timeout.to_string()),
                }
            }
        }
    }
}
