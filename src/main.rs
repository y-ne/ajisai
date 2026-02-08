mod db;
mod handlers;

use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let pool = db::connect().await;

    let app = Router::new()
        .route("/", get(handlers::root::root))
        .route("/webhook", post(handlers::webhook::receive))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
