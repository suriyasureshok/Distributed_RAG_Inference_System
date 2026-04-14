//! # Embedding Client
//!
//! Provide embedding generation for query text.

use shared::domain::errors::AppError;
use shared::resilience::retry::retry_with_backoff;
use shared::resilience::timeout::with_timeout;

use crate::config::client_config::ClientConfig;

/// Wrap embedding provider interactions with resilience policies.
pub struct EmbeddingClient {
    config: ClientConfig,
}

impl EmbeddingClient {
    /// Create a new embedding client.
    ///
    /// ## Arguments
    /// - `config`: Timeout and retry configuration.
    ///
    /// ## Returns
    /// A configured `EmbeddingClient`.
    pub fn new(config: ClientConfig) -> Self {
        Self { config }
    }

    /// Generate an embedding vector for a query.
    ///
    /// ## Arguments
    /// - `_query`: Query text to embed.
    ///
    /// ## Returns
    /// A dense embedding vector.
    ///
    /// ## Errors
    /// Returns `AppError` when timeout or retry attempts fail.
    pub async fn embed(&self, _query: &str) -> Result<Vec<f32>, AppError> {
        let config = &self.config;

        with_timeout(config.timeout, async {
            retry_with_backoff(config.max_retries, || async {
                Ok(vec![0.1, 0.2, 0.3]) // simulate
            })
            .await
        })
        .await
    }
}
