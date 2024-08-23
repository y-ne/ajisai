mod database;
mod models;
mod services;
mod handlers;

use database::db_pool;
use crate::handlers::user_handler::create_user_handler;
use dotenvy::dotenv;

use axum::{
    Router,
    routing::{get, post},
    Json,
};
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    dotenv().expect(".env not found");

    let pool = db_pool().await.expect("failed to crate pool");

    let app = Router::new()
        .route("/", get(root))
        .route("/user", post(create_user_handler))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Json<Value> {
    Json(json!({"data": "hello mom"}))
}