use dotenvy::dotenv;
use std::env;
use axum::{
    Router,
    routing::{get, post},
    http::StatusCode,
    Json
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;

#[tokio::main]
async fn main() {
    dotenv().expect(".env not found");

    let pg_url = env::var("DATABASE_URL"). expect("DATABASE_URL must be SET");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&pg_url).await
        .expect("failed to create database pool");

    let app = Router::new()
        .route("/", get(root))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Json<Value> {
    Json(json!({"data": "hello mom"}))
}