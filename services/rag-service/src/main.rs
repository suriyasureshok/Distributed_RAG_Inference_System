//! # rag-service
//!
//! Expose a retrieval-augmented generation service over HTTP.
//!
//! ## Overview
//! This binary wires transport, pipeline, clients, and shared utilities to serve
//! query requests at runtime.
//!
//! ## Features
//! - Axum-based HTTP endpoint for query execution.
//! - RAG pipeline with cache, retrieval, and generation stages.
//! - Shared resilience and observability integration.

mod application;
mod clients;
mod config;
mod services;
mod transport;
mod wiring;

use std::net::SocketAddr;

use axum::serve;
use tokio::net::TcpListener;

use transport::router::create_router;
use wiring::container::AppContainer;

/// Start the HTTP server and mount the RAG router.
#[tokio::main]
async fn main() {
    let app = AppContainer::new();
    let router = create_router(app.pipeline.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, router).await.unwrap();
}
