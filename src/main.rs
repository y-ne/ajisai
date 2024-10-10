mod database;
mod handlers;
mod models;
mod services;
mod utils;

use crate::handlers::user_handler::{
    create_user_handler, delete_user_handler, read_user_by_id_handler, read_users_handler,
    update_user_handler,
};
use database::db_pool;
use dotenvy::dotenv;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = db_pool().await.expect("Failed to create pool");

    let bcrypt_cost = std::env::var("BCRYPT_COST")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(12);

    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/users", get(read_users_handler))
        .route("/users", post(create_user_handler))
        .route("/users/:id", get(read_user_by_id_handler))
        .route("/users/:id", put(update_user_handler))
        .route("/users/:id", delete(delete_user_handler))
        .layer(cors)
        .with_state((pool, bcrypt_cost));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
