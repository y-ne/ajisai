mod database;
mod handlers;
mod models;
mod services;

use crate::handlers::user_handler::{create_user_handler, read_users_handler};
use database::db_pool;
use dotenvy::dotenv;
use handlers::user_handler::update_user_handler;

use axum::{
    routing::{get, post, put},
    Json, Router,
};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    dotenv().expect(".env not found");

    let pool = db_pool().await.expect("failed to crate pool");

    let app = Router::new()
        .route("/", get(root))
        .route("/user", get(read_users_handler))
        .route("/user", post(create_user_handler))
        .route("/user/:id", put(update_user_handler))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Json<Value> {
    Json(json!({"data": "hello mom"}))
}
