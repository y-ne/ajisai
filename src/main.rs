use dotenvy::dotenv;
use std::env;
use axum::{
    Router,
    routing::{get, post},
    http::StatusCode,
    Json,
    extract::State,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sqlx::postgres::{PgPoolOptions, PgPool};
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
        .route("/sum", get(sum_handler))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Json<Value> {
    Json(json!({"data": "hello mom"}))
}

async fn get_sum(pool: &PgPool) -> Result<i32, sqlx::Error> {
    let row = sqlx::query("SELECT 1 + 1 as sum")
        .fetch_one(pool)
        .await?;
    
    let sum: i32 = row.get("sum");
    Ok(sum)
}

async fn sum_handler(State(pool): State<PgPool>) -> Result<Json<Value>, (StatusCode, String)> {
    match get_sum(&pool).await {
        Ok(sum) => Ok(Json(json!({"sum": sum}))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}