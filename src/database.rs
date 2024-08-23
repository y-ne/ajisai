use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

pub async fn db_pool() -> Result<PgPool, sqlx::Error> {
    let pg_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&pg_url)
        .await
}