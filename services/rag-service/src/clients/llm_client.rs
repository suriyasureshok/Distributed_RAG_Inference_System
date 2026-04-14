use std::sync::Arc;

use shared::domain::errors::AppError;
use shared::observability::logger::{log_error, log_info};
use shared::resilience::circuit_breaker::CircuitBreaker;
use shared::resilience::retry::retry_with_backoff;
use shared::resilience::timeout::with_timeout;

use crate::config::client_config::ClientConfig;

pub struct LLMClient {
    config: ClientConfig,
    circuit_breaker: Arc<CircuitBreaker>,
}

impl LLMClient {
    pub fn new(config: ClientConfig, circuit_breaker: Arc<CircuitBreaker>) -> Self {
        Self {
            config,
            circuit_breaker,
        }
    }

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

    async fn fallback(&self, _context: Vec<String>) -> Result<String, AppError> {
        // fallback logic (cheap model / default response)
        Ok("Fallback response".to_string())
    }
}
