use std::sync::Arc;

use crate::services::cache_service::CacheService;
use crate::services::coalescing::Coalescer;

use crate::clients::embedding_client::EmbeddingClient;
use crate::clients::llm_client::LLMClient;
use crate::clients::vector_client::VectorClient;

use shared::domain::errors::AppError;
use shared::observability::logger::{log_error, log_info};
use shared::observability::metrics::METRICS;
use shared::utils::semantic_key::semantic_key;

pub struct RagPipeline {
    pub cache: Arc<CacheService>,
    pub coalescer: Arc<Coalescer<String>>,

    pub embedding_client: Arc<EmbeddingClient>,
    pub vector_client: Arc<VectorClient>,
    pub llm_client: Arc<LLMClient>,
}

impl RagPipeline {
    pub async fn handle_query(&self, query: String) -> Result<String, AppError> {
        METRICS.inc_requests();
        log_info("Incoming request");
        let start = std::time::Instant::now();

        let key = semantic_key(&query);

        let result = self
            .coalescer
            .run(key, || async {
                self.execute_pipeline(query.clone())
                    .await
                    .unwrap_or_default()
            })
            .await;

        let latency = start.elapsed().as_millis();
        log_info(&format!("Request completed in {} ms", latency));

        Ok(result)
    }

    async fn execute_pipeline(&self, query: String) -> Result<String, AppError> {
        let cache_key = semantic_key(&query);

        // Cache
        if let Some(cached) = self.cache.get(&cache_key).await {
            METRICS.inc_cache_hits();
            log_info("Cache hit");
            return Ok(cached);
        }

        // Embedding
        let embedding = self.embedding_client.embed(&query).await?;

        // Vector Search
        let docs = self.vector_client.search(embedding).await?;

        // LLM
        let response = self.llm_client.generate(docs).await.map_err(|err| {
            log_error("LLM failed");
            METRICS.inc_errors();
            err
        })?;

        // Cache write
        self.cache.set(&cache_key, response.clone()).await;

        Ok(response)
    }
}
