use axum::{Router, routing::post};
use std::sync::Arc;

use crate::application::rag_pipeline::RagPipeline;
use crate::transport::handler::handle_query;

pub fn create_router(pipeline: Arc<RagPipeline>) -> Router {
    Router::new()
        .route("/query", post(handle_query))
        .with_state(pipeline)
}
