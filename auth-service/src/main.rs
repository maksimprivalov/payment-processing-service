mod config;
mod db;
mod models;
mod handlers;
mod auth;
mod error;

use axum::{Router, routing::post};
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use config::Config;
use db::init_db;
use handlers::{register, login};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::from_env();
    let db = init_db(&config.database_url).await;

    let state = (db, config.jwt_secret);

    let app = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    println!("Auth service running on 8080");

    axum::serve(listener, app)
        .await
        .unwrap();
}
