use crate::models::user::{CreateUserRequest, UpdateUserRequest, User};
use crate::services::user_service;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn read_users_handler(
    State((pool, _)): State<(PgPool, u32)>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let users = user_service::read_users(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(users))
}

pub async fn read_user_by_id_handler(
    State((pool, _)): State<(PgPool, u32)>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, (StatusCode, String)> {
    let user = user_service::read_user_by_id(&pool, id)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "User not found".to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        })?;
    Ok(Json(user))
}

pub async fn create_user_handler(
    State((pool, bcrypt_cost)): State<(PgPool, u32)>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, (StatusCode, String)> {
    let user = user_service::create_user(
        &pool,
        &payload.username,
        &payload.password,
        payload.role.unwrap_or_default(),
        bcrypt_cost,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(user))
}

pub async fn update_user_handler(
    State((pool, bcrypt_cost)): State<(PgPool, u32)>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<User>, (StatusCode, String)> {
    let user = user_service::update_user(
        &pool,
        id,
        payload.username.as_deref(),
        payload.password.as_deref(),
        payload.status,
        payload.role,
        bcrypt_cost,
    )
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "User not found".to_string()),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    })?;
    Ok(Json(user))
}

pub async fn delete_user_handler(
    State((pool, _)): State<(PgPool, u32)>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let deleted = user_service::delete_user(&pool, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((StatusCode::NOT_FOUND, "User not found".to_string()))
    }
}
