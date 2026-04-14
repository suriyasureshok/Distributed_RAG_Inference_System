//! # RAG Pipeline
//!
//! Orchestrate end-to-end query execution for retrieval-augmented generation.
//!
//! ## Responsibilities
//! - Normalize and coalesce duplicate requests.
//! - Read-through and write-back cache interactions.
//! - Invoke embedding, retrieval, and LLM generation clients.
//! - Emit basic observability signals.

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

/// Coordinate all stages of the RAG execution flow.
///
/// ## Fields
/// - `cache`: Two-layer cache service for request/response reuse.
/// - `coalescer`: In-flight deduplication for identical semantic keys.
/// - `embedding_client`: Embedding provider adapter.
/// - `vector_client`: Vector retrieval adapter.
/// - `llm_client`: Generation provider adapter.
pub struct RagPipeline {
    /// Two-layer cache service for request/response reuse.
    pub cache: Arc<CacheService>,
    /// In-flight deduplication for identical semantic keys.
    pub coalescer: Arc<Coalescer<String>>,

    /// Embedding provider adapter.
    pub embedding_client: Arc<EmbeddingClient>,
    /// Vector retrieval adapter.
    pub vector_client: Arc<VectorClient>,
    /// Generation provider adapter.
    pub llm_client: Arc<LLMClient>,
}

impl RagPipeline {
    /// Execute a query through the coalesced RAG pipeline.
    ///
    /// ## Arguments
    /// - `query`: Raw user query text.
    ///
    /// ## Returns
    /// The generated answer string.
    ///
    /// ## Errors
    /// Returns `AppError` when an unrecoverable failure occurs.
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

    /// Execute pipeline stages in order for a single query.
    ///
    /// ## Arguments
    /// - `query`: Raw user query text.
    ///
    /// ## Returns
    /// The generated answer string.
    ///
    /// ## Errors
    /// Returns `AppError` when any pipeline stage fails.
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
