use axum::{extract::State, http::StatusCode, Json};
// use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::models::user::{User, UserRequest};
use crate::services::user_service::{create_user, read_users};

pub async fn read_users_handler(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    match read_users(&pool).await {
        Ok(users) => Ok(Json(users)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn create_user_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<UserRequest>,
) -> Result<Json<User>, (StatusCode, String)> {
    match create_user(&pool, &payload.username, &payload.password).await {
        Ok(user) => Ok(Json(User {
            id: user.id,
            username: user.username,
            password: user.password,
            status: user.status,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
