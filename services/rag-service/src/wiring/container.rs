use std::sync::Arc;

use crate::services::cache_service::CacheService;
use crate::services::coalescing::Coalescer;

use crate::clients::embedding_client::EmbeddingClient;
use crate::clients::llm_client::LLMClient;
use crate::clients::vector_client::VectorClient;

use crate::application::rag_pipeline::RagPipeline;

use crate::config::app_config::AppConfig;
use crate::config::client_config::ClientConfig;

use shared::resilience::circuit_breaker::CircuitBreaker;

pub struct AppContainer {
    pub pipeline: Arc<RagPipeline>,
}

impl AppContainer {
    pub fn new() -> Self {
        // 1. Load config (hardcoded for now)
        let app_config = AppConfig {
            embedding_timeout: std::time::Duration::from_millis(200),
            vector_timeout: std::time::Duration::from_millis(300),
            llm_timeout: std::time::Duration::from_secs(2),
            max_retries: 3,
            circuit_breaker_threshold: 5,
            circuit_breaker_timeout: std::time::Duration::from_secs(10),
        };

        let embedding_client_config = ClientConfig {
            timeout: app_config.embedding_timeout,
            max_retries: app_config.max_retries,
        };

        let vector_client_config = ClientConfig {
            timeout: app_config.vector_timeout,
            max_retries: app_config.max_retries,
        };

        let llm_client_config = ClientConfig {
            timeout: app_config.llm_timeout,
            max_retries: app_config.max_retries,
        };

        // 2. Resilience
        let circuit_breaker = Arc::new(CircuitBreaker::new(
            app_config.circuit_breaker_threshold,
            app_config.circuit_breaker_timeout,
        ));

        // 3. Clients
        let embedding_client = Arc::new(EmbeddingClient::new(embedding_client_config));
        let vector_client = Arc::new(VectorClient::new(vector_client_config));
        let llm_client = Arc::new(LLMClient::new(llm_client_config, circuit_breaker));

        // 4. Cache (mock for now)
        let cache_client = Arc::new(crate::clients::cache_client::MockCacheClient::new());
        let cache_service = Arc::new(CacheService::new(cache_client));

        // 5. Coalescer
        let coalescer = Arc::new(Coalescer::new());

        // 6. Pipeline
        let pipeline = Arc::new(RagPipeline {
            cache: cache_service,
            coalescer,
            embedding_client,
            vector_client,
            llm_client,
        });

        Self { pipeline }
    }
}
