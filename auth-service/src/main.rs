mod config;
mod db;
mod models;
mod handlers;
mod auth;

use axum::{Router, routing::get};
use std::net::SocketAddr;
use tokio::net::TcpListener;

async fn healthcheck() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(healthcheck));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Running on {}", addr);

    axum::serve(listener, app)
        .await
        .unwrap();
}
