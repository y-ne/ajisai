use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use serde_json::{json, Map, Value};
use sqlx::PgPool;

pub async fn receive(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    let headers: Value = headers
        .iter()
        .filter_map(|(k, v)| Some((k.to_string(), Value::String(v.to_str().ok()?.to_string()))))
        .collect::<Map<String, Value>>()
        .into();

    let key = format!("WEBHOOK:{:x}", uuid::Uuid::new_v4());
    let value = json!({ "headers": headers, "body": body });

    sqlx::query!(
        "INSERT INTO key_value (key, value) VALUES ($1, $2)",
        key,
        value
    )
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({ "status": "ok" })))
}
