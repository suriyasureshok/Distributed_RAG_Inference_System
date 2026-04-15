//! # HTTP Handlers
//!
//! Implement request/response handling for embedding endpoints.

use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use shared::domain::errors::AppError;
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

#[derive(Serialize)]
/// Represent the JSON error response payload for embedding failures.
///
/// ## Fields
/// - `error`: Human-readable error description.
struct ErrorResponse {
    /// Human-readable error description.
    error: String,
}

/// Convert an `AppError` into an HTTP response.
///
/// ## Arguments
/// - `err`: Application error propagated from service layer.
///
/// ## Returns
/// HTTP response with mapped status code and JSON error body.
fn app_error_to_response(err: AppError) -> Response {
    let status = match err {
        AppError::CacheMiss => StatusCode::NOT_FOUND,
        AppError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        AppError::CircuitOpen => StatusCode::SERVICE_UNAVAILABLE,
        AppError::EmbeddingError(_) => StatusCode::BAD_REQUEST,
        AppError::CacheError(_) | AppError::VectorSearchError(_) | AppError::LLMError(_) => {
            StatusCode::BAD_GATEWAY
        }
        AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };

    let body = Json(ErrorResponse {
        error: err.to_string(),
    });

    (status, body).into_response()
}

/// Handle embedding requests and return generated vectors.
///
/// ## Arguments
/// - `State(service)`: Shared embedding service state.
/// - `Json(req)`: Request payload containing query text.
///
/// ## Returns
/// JSON response with the generated embedding vector.
///
/// ## Errors
/// Returns an HTTP error response mapped from `AppError` when embedding generation fails.
pub async fn generate_embedding(
    State(service): State<Arc<EmbeddingService>>,
    Json(req): Json<EmbeddingRequest>,
) -> Result<Json<EmbeddingResponse>, Response> {
    let embedding = service
        .generate(&req.query)
        .await
        .map_err(app_error_to_response)?;

    Ok(Json(EmbeddingResponse { embedding }))
}
