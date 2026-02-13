mod config;
mod db;
mod models;
mod handlers;
mod error;
mod auth;
mod middleware;

use axum::{
    Router,
    routing::{post, get},
    middleware as axum_middleware,
};
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use config::Config;
use db::init_db;
use handlers::*;
use middleware::auth_middleware;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::from_env();
    let db = init_db(&config.database_url).await;

    let state = (db, config.jwt_secret);

    let protected = Router::new()
        .route("/accounts", post(create_account))
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    let app = Router::new()
        .merge(protected)
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8081")
        .await
        .unwrap();

    println!("Account service running on 8081");

    axum::serve(listener, app)
        .await
        .unwrap();
}
