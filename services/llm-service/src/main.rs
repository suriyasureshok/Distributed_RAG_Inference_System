//! # llm-service
//!
//! Expose LLM text generation over HTTP.
//!
//! ## Overview
//! This binary wires providers, service orchestration, transport handlers, and
//! resilience utilities into a runnable HTTP server.
//!
//! ## Features
//! - Axum-based HTTP endpoint for text generation.
//! - Primary and fallback generation providers.
//! - Retry, timeout, and circuit-breaker guarded execution.

mod providers;
mod service;
mod transport;
mod wiring;

use axum::serve;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use transport::routes::create_router;
use wiring::container::AppContainer;

/// Start the HTTP server and mount LLM generation routes.
///
/// ## Panics
/// Panics if socket binding or server serving fails.
#[tokio::main]
async fn main() {
    let app = AppContainer::new();

    let router = create_router(app.llm_service.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 4002));
    println!("LLM Service running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, router).await.unwrap();
}
