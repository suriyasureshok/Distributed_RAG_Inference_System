use shared::domain::errors::AppError;
use shared::resilience::retry::retry_with_backoff;
use shared::resilience::timeout::with_timeout;

use crate::config::client_config::ClientConfig;

pub struct VectorClient {
    config: ClientConfig,
}

impl VectorClient {
    pub fn new(config: ClientConfig) -> Self {
        Self { config }
    }

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
