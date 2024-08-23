use sqlx::PgPool;
use crate::models::user::User;

pub async fn read_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, password, status
        FROM users
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(users)
}

pub async fn create_user(
    pool: &PgPool,
    username: &str,
    password: &str,
    status: Option<bool>,
) -> Result<User, sqlx::Error> {
    let status = status.unwrap_or(true);

    let row = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, password, status) 
        VALUES ($1, $2, $3)
        RETURNING id, username, password, status
        "#,
        username,
        password,
        status
    )
    .fetch_one(pool)
    .await?;

    Ok(row)
}