use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::services::user_service::{read_users ,create_user};
use crate::models::user::User;

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

pub async fn read_users_handler(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    match read_users(&pool).await {
        Ok(users) => Ok(Json(users)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
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