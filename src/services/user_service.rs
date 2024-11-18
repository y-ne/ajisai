use anyhow::Result;
use bcrypt::hash;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user::{User, UserRole, UserStatus};

#[derive(Clone)]
pub struct UserService {
    pool: PgPool,
    bcrypt_cost: u32,
}

impl UserService {
    pub fn new(pool: PgPool, bcrypt_cost: u32) -> Self {
        Self { pool, bcrypt_cost }
    }

    pub async fn list(&self) -> Result<Vec<User>> {
        Ok(sqlx::query_as!(
            User,
            r#"
            SELECT id, username, password, 
                   status as "status: UserStatus", 
                   role as "role: UserRole",
                   created_at, updated_at
            FROM users
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<User> {
        Ok(sqlx::query_as!(
            User,
            r#"
            SELECT id, username, password,
                   status as "status: UserStatus",
                   role as "role: UserRole",
                   created_at, updated_at
            FROM users WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn create(&self, username: &str, password: &str, role: UserRole) -> Result<User> {
        let hashed_password = hash(password, self.bcrypt_cost)?;

        Ok(sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, password, role, status)
            VALUES ($1, $2, $3, $4)
            RETURNING id, username, password,
                      status as "status: UserStatus",
                      role as "role: UserRole",
                      created_at, updated_at
            "#,
            username,
            hashed_password,
            role as UserRole,
            UserStatus::Pending as UserStatus
        )
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn update(
        &self,
        id: Uuid,
        username: Option<&str>,
        password: Option<&str>,
        status: Option<UserStatus>,
        role: Option<UserRole>,
    ) -> Result<User> {
        let hashed_password = if let Some(pwd) = password {
            Some(hash(pwd, self.bcrypt_cost)?)
        } else {
            None
        };

        Ok(sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET username = COALESCE($2, username),
                password = COALESCE($3, password),
                status = COALESCE($4, status),
                role = COALESCE($5, role),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, username, password,
                      status as "status: UserStatus",
                      role as "role: UserRole",
                      created_at, updated_at
            "#,
            id,
            username,
            hashed_password,
            status as Option<UserStatus>,
            role as Option<UserRole>
        )
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn delete(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}