use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::Error as SqlxError;
use uuid::Uuid;

use crate::{
    models::user::{CreateUserRequest, UpdateUserRequest, User},
    services::user_service::UserService,
};

fn handle_error(err: anyhow::Error) -> (StatusCode, String) {
    if let Some(sqlx_err) = err.downcast_ref::<SqlxError>() {
        match sqlx_err {
            SqlxError::RowNotFound => (StatusCode::NOT_FOUND, "User not found".into()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
        }
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }
}

pub async fn read_users(
    State(service): State<UserService>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    service.list().await.map(Json).map_err(handle_error)
}

pub async fn read_user_by_id(
    State(service): State<UserService>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, (StatusCode, String)> {
    service.find_by_id(id).await.map(Json).map_err(handle_error)
}

pub async fn create_user(
    State(service): State<UserService>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, (StatusCode, String)> {
    service
        .create(
            &payload.username,
            &payload.password,
            payload.role.unwrap_or_default(),
        )
        .await
        .map(Json)
        .map_err(handle_error)
}

pub async fn update_user(
    State(service): State<UserService>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<User>, (StatusCode, String)> {
    service
        .update(
            id,
            payload.username.as_deref(),
            payload.password.as_deref(),
            payload.status,
            payload.role,
        )
        .await
        .map(Json)
        .map_err(handle_error)
}

pub async fn delete_user(
    State(service): State<UserService>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let deleted = service.delete(id).await.map_err(handle_error)?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((StatusCode::NOT_FOUND, "User not found".into()))
    }
}