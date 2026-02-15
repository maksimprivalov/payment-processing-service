mod models;
mod handlers;
mod db;
mod config;
mod auth;
mod error;
mod middleware;

use axum::{
    Router,
    routing::post,
    middleware as axum_middleware,
};
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use config::Config;
use db::init_db;
use handlers::create_payment;
use middleware::auth_middleware;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::from_env();

    let db = init_db(&config.database_url).await;

    let state = (db, config.jwt_secret);

    let protected_routes = Router::new()
        .route("/payments", post(create_payment))
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    let app = Router::new()
        .merge(protected_routes)
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8082")
        .await
        .unwrap();

    println!("Payment service running on 8082");

    axum::serve(listener, app)
        .await
        .unwrap();
}
