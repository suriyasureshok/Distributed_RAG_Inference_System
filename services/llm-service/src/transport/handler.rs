//! # HTTP Handlers
//!
//! Implement request/response handling for generation endpoints.

use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::service::llm_service::LLMService;

#[derive(Deserialize)]
/// Represent the JSON request payload for LLM generation.
///
/// ## Fields
/// - `context`: Prompt or context segments.
pub struct LLMRequest {
    /// Prompt or context segments.
    pub context: Vec<String>,
}

#[derive(Serialize)]
/// Represent the JSON response payload for LLM generation.
///
/// ## Fields
/// - `answer`: Generated answer text.
pub struct LLMResponse {
    /// Generated answer text.
    pub answer: String,
}

/// Handle generation requests and return answer text.
///
/// ## Arguments
/// - `State(service)`: Shared LLM service state.
/// - `Json(req)`: Request payload containing generation context.
///
/// ## Returns
/// JSON response with generated answer on success.
///
/// ## Errors
/// Returns `500 Internal Server Error` with JSON body when generation fails.
pub async fn generate(
    State(service): State<Arc<LLMService>>,
    Json(req): Json<LLMRequest>,
) -> Result<Json<LLMResponse>, (StatusCode, Json<LLMResponse>)> {
    let result = service.generate(req.context).await;

    match result {
        Ok(answer) => Ok(Json(LLMResponse { answer })),
        Err(e) => {
            tracing::error!(error = %e, "LLM generation failed");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(LLMResponse {
                    answer: "Internal error".to_string(),
                }),
            ))
        }
    }
}
