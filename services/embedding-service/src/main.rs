//! # embedding-service
//!
//! Expose query-to-vector embedding generation over HTTP.
//!
//! ## Overview
//! This binary wires the embedding domain service into transport handlers and
//! starts an Axum server for runtime embedding requests.
//!
//! ## Features
//! - Axum-based HTTP endpoint for embedding generation.
//! - Deterministic mock embedding generation for local development.
//! - Lightweight dependency container for shared state.

mod transport;
mod service;
mod config;
mod wiring;

use std::net::SocketAddr;

use axum::serve;
use tokio::net::TcpListener;

use wiring::container::AppContainer;
use transport::routes::create_router;

/// Start the HTTP server and mount embedding routes.
///
/// ## Panics
/// Panics if socket binding or server serving fails.
#[tokio::main]
async fn main() {
    let app = AppContainer::new();

    let router = create_router(app.embedding_service.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 4001));
    println!("Embedding Service running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, router).await.unwrap();
}