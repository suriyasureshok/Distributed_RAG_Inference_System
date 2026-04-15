//! # Embedding Service
//!
//! Implement query-to-vector embedding generation.
//!
//! ## Responsibilities
//! - Convert query text into normalized float vectors.
//! - Simulate provider latency for local integration flows.

use std::time::Duration;

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
    /// A normalized embedding vector with up to 10 dimensions.
    ///
    /// ## Performance
    /// Runs in O(n) over query bytes while capping output length at 10.
    pub async fn generate(&self, query: &str) -> Vec<f32> {
        tokio::time::sleep(Duration::from_millis(50)).await;

        // Normalize bytes into a bounded deterministic vector for local testing.
        query
            .bytes()
            .map(|b| (b as f32) / 255.0)
            .take(10)
            .collect()
    }
}