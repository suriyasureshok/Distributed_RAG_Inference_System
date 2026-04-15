//! # Router
//!
//! Build the HTTP router for llm-service.

use axum::{Router, routing::post};
use std::sync::Arc;

use crate::service::llm_service::LLMService;
use crate::transport::handler::generate;

/// Create the application router with shared LLM service state.
///
/// ## Arguments
/// - `service`: Shared LLM service state for handler execution.
///
/// ## Returns
/// Configured `Router` with mounted generation endpoint.
pub fn create_router(service: Arc<LLMService>) -> Router {
    Router::new()
        .route("/generate", post(generate))
        .with_state(service)
}
