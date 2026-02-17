mod config;
mod handlers;
mod error;
mod models;

use axum::{Router, routing::post};
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use config::Config;
use handlers::transfer;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::from_env();

    let app = Router::new()
        .route("/transfer", post(transfer))
        .layer(CorsLayer::permissive())
        .with_state(config);

    let listener = TcpListener::bind("0.0.0.0:8085")
        .await
        .unwrap();

    println!("Saga running on 8085");

    axum::serve(listener, app)
        .await
        .unwrap();
}
