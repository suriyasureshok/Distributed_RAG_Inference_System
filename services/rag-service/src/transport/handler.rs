//! # HTTP Handlers
//!
//! Implement request/response handling for transport endpoints.

use axum::Json;
use axum::extract::State;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::application::rag_pipeline::RagPipeline;

#[derive(Deserialize)]
/// Represent the JSON request payload for query execution.
///
/// ## Fields
/// - `query`: User question text.
pub struct QueryRequest {
    /// User question text.
    pub query: String,
}

#[derive(Serialize)]
/// Represent the JSON response payload for query execution.
///
/// ## Fields
/// - `answer`: Generated answer text.
pub struct QueryResponse {
    /// Generated answer text.
    pub answer: String,
}

/// Handle query requests and return generated answers.
///
/// ## Arguments
/// - `State(pipeline)`: Shared application pipeline state.
/// - `Json(req)`: Request payload containing user query.
///
/// ## Returns
/// JSON response with generated answer or fallback error message.
pub async fn handle_query(
    State(pipeline): State<Arc<RagPipeline>>,
    Json(req): Json<QueryRequest>,
) -> Json<QueryResponse> {
    let result = pipeline.handle_query(req.query).await;

    match result {
        Ok(answer) => Json(QueryResponse { answer }),
        Err(_) => Json(QueryResponse {
            answer: "Internal error".to_string(),
        }),
    }
}
