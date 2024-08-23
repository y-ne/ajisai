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

#[tokio::main]
async fn main() {
    dotenv().expect(".env not found");

    let app = Router::new()
        .route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Json<Value> {
    Json(json!({"data": "hello mom"}))
}