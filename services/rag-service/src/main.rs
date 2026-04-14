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

#[tokio::main]
async fn main() {
    let app = AppContainer::new();
    let router = create_router(app.pipeline.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, router).await.unwrap();
}
