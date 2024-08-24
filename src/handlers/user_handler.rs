use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
// use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::services::user_service::{create_user, read_user_by_id, read_users, update_user};
use crate::{
    models::user::{User, UserRequest, UserUpdateRequest},
    services::user_service::delete_user,
};

pub async fn read_users_handler(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    match read_users(&pool).await {
        Ok(users) => Ok(Json(users)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn read_user_by_id_handler(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<User>, (StatusCode, String)> {
    match read_user_by_id(&pool, id).await {
        Ok(user) => Ok(Json(user)),
        Err(sqlx::Error::RowNotFound) => Err((StatusCode::NOT_FOUND, "User not found".to_string())),
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

pub async fn update_user_handler(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UserUpdateRequest>,
) -> Result<Json<User>, (StatusCode, String)> {
    match update_user(
        &pool,
        id,
        &payload.username,
        &payload.password,
        payload.status,
    )
    .await
    {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err((StatusCode::NOT_MODIFIED, e.to_string())),
    }
}

pub async fn delete_user_handler(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    match delete_user(&pool, id).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err((StatusCode::NOT_FOUND, "User not found.".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
