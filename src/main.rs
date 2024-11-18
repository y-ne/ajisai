use anyhow::Result;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::net::SocketAddr;  // Add this import
use tower_http::cors::CorsLayer;
use tracing::info;

mod database;
mod handlers;
mod models;
mod services;

use handlers::user_handler::{create_user, delete_user, read_user_by_id, read_users, update_user};
use services::user_service::UserService;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Create database pool
    let pool = database::create_pool().await?;

    // Create user service
    let bcrypt_cost = std::env::var("BCRYPT_COST")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(12);
    
    let user_service = UserService::new(pool, bcrypt_cost);

    // Build router
    let app = Router::new()
        .route("/users", get(read_users))
        .route("/users", post(create_user))
        .route("/users/:id", get(read_user_by_id))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
        .layer(CorsLayer::permissive())
        .with_state(user_service);

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("🚀 Server starting on {addr}");
    
    axum::serve(
        tokio::net::TcpListener::bind(addr).await?,
        app
    ).await?;

    Ok(())
}