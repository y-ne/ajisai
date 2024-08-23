use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use crate::services::user_service::create_user;
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    username: String,
    password: String,
    status: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    id: i32,
    username: String,
    password: String,
    status: Option<bool>,
}

pub async fn create_user_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    match create_user(&pool, &payload.username, &payload.password, payload.status).await {
        Ok(user) => Ok(Json(UserResponse {
            id: user.id,
            username: user.username,
            password: user.password,
            status: user.status,
        })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}