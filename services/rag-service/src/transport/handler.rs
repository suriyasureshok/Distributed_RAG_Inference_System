use axum::Json;
use axum::extract::State;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::application::rag_pipeline::RagPipeline;

#[derive(Deserialize)]
pub struct QueryRequest {
    pub query: String,
}

#[derive(Serialize)]
pub struct QueryResponse {
    pub answer: String,
}

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
