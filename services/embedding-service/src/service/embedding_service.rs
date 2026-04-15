//! # Embedding Service
//!
//! Implement query-to-vector embedding generation.
//!
//! ## Responsibilities
//! - Convert query text into normalized float vectors.
//! - Simulate provider latency for local integration flows.

use std::time::Duration;

use shared::domain::errors::AppError;

/// Provide embedding generation behavior.
pub struct EmbeddingService;

impl EmbeddingService {
    /// Create a new embedding service instance.
    ///
    /// ## Returns
    /// A new `EmbeddingService`.
    pub fn new() -> Self {
        Self
    }

    /// Generate a deterministic embedding vector from a query.
    ///
    /// ## Arguments
    /// - `query`: Input query text.
    ///
    /// ## Returns
    /// A normalized embedding vector with 10 dimensions.
    ///
    /// ## Errors
    /// Returns `AppError::EmbeddingError` when the query is empty.
    ///
    /// ## Performance
    /// Runs in O(n) over query bytes while capping output length at 10.
    pub async fn generate(&self, query: &str) -> Result<Vec<f32>, AppError> {
        if query.trim().is_empty() {
            return Err(AppError::EmbeddingError(
                "query must not be empty".to_string(),
            ));
        }

        tokio::time::sleep(Duration::from_millis(50)).await;

        const EMBEDDING_DIM: usize = 10;

        // Normalize bytes into a fixed-dimension deterministic vector for local testing.
        let mut embedding: Vec<f32> = query
            .bytes()
            .map(|b| (b as f32) / 255.0)
            .take(EMBEDDING_DIM)
            .collect();

        // Pad with zeros to ensure consistent dimension
        embedding.resize(EMBEDDING_DIM, 0.0);
        Ok(embedding)
    }
}
