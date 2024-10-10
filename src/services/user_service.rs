use crate::models::user::{User, UserRole, UserStatus};
use bcrypt::hash;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn read_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        SELECT id, username, password, status as "status: UserStatus", role as "role: UserRole", created_at, updated_at
        FROM users
        "#
    )
    .fetch_all(pool)
    .await
}

pub async fn read_user_by_id(pool: &PgPool, id: Uuid) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        SELECT id, username, password, status as "status: UserStatus", role as "role: UserRole", created_at, updated_at
        FROM users
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn create_user(
    pool: &PgPool,
    username: &str,
    password: &str,
    role: UserRole,
    bcrypt_cost: u32,
) -> Result<User, sqlx::Error> {
    let hashed_password = hash(password, bcrypt_cost).expect("Failed to hash password");

    sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, password, role, status)
        VALUES ($1, $2, $3, $4)
        RETURNING id, username, password, status as "status: UserStatus", role as "role: UserRole", created_at, updated_at
        "#,
        username,
        hashed_password,
        role as UserRole,
        UserStatus::Pending as UserStatus
    )
    .fetch_one(pool)
    .await
}

pub async fn update_user(
    pool: &PgPool,
    id: Uuid,
    username: Option<&str>,
    password: Option<&str>,
    status: Option<UserStatus>,
    role: Option<UserRole>,
    bcrypt_cost: u32,
) -> Result<User, sqlx::Error> {
    let hashed_password = if let Some(new_password) = password {
        Some(hash(new_password, bcrypt_cost).expect("Failed to hash password"))
    } else {
        None
    };

    sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET 
            username = COALESCE($2, username),
            password = COALESCE($3, password),
            status = COALESCE($4, status),
            role = COALESCE($5, role),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, username, password, status as "status: UserStatus", role as "role: UserRole", created_at, updated_at
        "#,
        id,
        username,
        hashed_password,
        status as Option<UserStatus>,
        role as Option<UserRole>
    )
    .fetch_one(pool)
    .await
}

pub async fn delete_user(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM users
        WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}
