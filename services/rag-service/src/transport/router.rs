//! # Router
//!
//! Build the HTTP router for the RAG service.

use axum::{Router, routing::post};
use std::sync::Arc;

use crate::application::rag_pipeline::RagPipeline;
use crate::transport::handler::handle_query;

/// Create the application router with shared pipeline state.
///
/// ## Arguments
/// - `pipeline`: Shared pipeline state for handler execution.
///
/// ## Returns
/// Configured `Router` with mounted endpoints.
pub fn create_router(pipeline: Arc<RagPipeline>) -> Router {
    Router::new()
        .route("/query", post(handle_query))
        .with_state(pipeline)
}
