use shared::domain::errors::AppError;
use shared::resilience::retry::retry_with_backoff;
use shared::resilience::timeout::with_timeout;

use crate::config::client_config::ClientConfig;

pub struct EmbeddingClient {
    config: ClientConfig,
}

impl EmbeddingClient {
    pub fn new(config: ClientConfig) -> Self {
        Self { config }
    }

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
