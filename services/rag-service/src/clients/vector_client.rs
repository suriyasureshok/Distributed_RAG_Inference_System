//! # Vector Client
//!
//! Provide vector similarity search for embedded queries.

use shared::domain::errors::AppError;
use shared::resilience::retry::retry_with_backoff;
use shared::resilience::timeout::with_timeout;

use crate::config::client_config::ClientConfig;

/// Wrap vector search provider interactions with resilience policies.
pub struct VectorClient {
    config: ClientConfig,
}

impl VectorClient {
    /// Create a new vector client.
    ///
    /// ## Arguments
    /// - `config`: Timeout and retry configuration.
    ///
    /// ## Returns
    /// A configured `VectorClient`.
    pub fn new(config: ClientConfig) -> Self {
        Self { config }
    }

    /// Search related documents using an embedding vector.
    ///
    /// ## Arguments
    /// - `_embedding`: Query embedding vector.
    ///
    /// ## Returns
    /// A list of retrieved document payloads.
    ///
    /// ## Errors
    /// Returns `AppError` when timeout or retry attempts fail.
    pub async fn search(&self, _embedding: Vec<f32>) -> Result<Vec<String>, AppError> {
        let config = &self.config;

        with_timeout(config.timeout, async {
            retry_with_backoff(config.max_retries, || async {
                Ok(vec!["doc1".into(), "doc2".into()])
            })
            .await
        })
        .await
    }
}
