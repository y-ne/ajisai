use crate::models::user::User;
use sqlx::PgPool;

pub async fn read_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, password, status, created_at, updated_at
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
) -> Result<User, sqlx::Error> {
    let row = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, password)
        VALUES ($1, $2)
        RETURNING id, username, password, status, created_at, updated_at
        "#,
        username,
        password
    )
    .fetch_one(pool)
    .await?;

    Ok(row)
}
