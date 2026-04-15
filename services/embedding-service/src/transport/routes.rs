//! # Router
//!
//! Build the HTTP router for embedding-service.

use axum::{routing::post, Router};
use std::sync::Arc;

use crate::service::embedding_service::EmbeddingService;
use crate::transport::handler::generate_embedding;

/// Create the application router with shared embedding service state.
///
/// ## Arguments
/// - `service`: Shared embedding service used by endpoint handlers.
///
/// ## Returns
/// Configured `Router` with mounted embedding endpoints.
pub fn create_router(service: Arc<EmbeddingService>) -> Router {
    Router::new()
        .route("/embed", post(generate_embedding))
        .with_state(service)
}