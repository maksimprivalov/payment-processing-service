mod config;
mod db;
mod models;
mod handlers;
mod error;

use axum::{Router, routing::post};
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use config::Config;
use db::init_db;
use handlers::create_event;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::from_env();
    let db = init_db(&config.database_url).await;

    let app = Router::new()
        .route("/audit", post(create_event))
        .layer(CorsLayer::permissive())
        .with_state(db);

    let listener = TcpListener::bind("0.0.0.0:8084")
        .await
        .unwrap();

    println!("Audit service running on 8084");

    axum::serve(listener, app)
        .await
        .unwrap();
}