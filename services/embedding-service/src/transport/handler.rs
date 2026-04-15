//! # HTTP Handlers
//!
//! Implement request/response handling for embedding endpoints.

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::service::embedding_service::EmbeddingService;

#[derive(Deserialize)]
/// Represent the JSON request payload for embedding generation.
///
/// ## Fields
/// - `query`: Input query text to embed.
pub struct EmbeddingRequest {
    /// Input query text to embed.
    pub query: String,
}

#[derive(Serialize)]
/// Represent the JSON response payload for embedding generation.
///
/// ## Fields
/// - `embedding`: Generated embedding vector values.
pub struct EmbeddingResponse {
    /// Generated embedding vector values.
    pub embedding: Vec<f32>,
}

/// Handle embedding requests and return generated vectors.
///
/// ## Arguments
/// - `State(service)`: Shared embedding service state.
/// - `Json(req)`: Request payload containing query text.
///
/// ## Returns
/// JSON response with the generated embedding vector.
pub async fn generate_embedding(
    State(service): State<Arc<EmbeddingService>>,
    Json(req): Json<EmbeddingRequest>,
) -> Json<EmbeddingResponse> {
    let embedding = service.generate(&req.query).await;

    Json(EmbeddingResponse { embedding })
}