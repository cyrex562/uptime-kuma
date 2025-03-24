use sqlx::sqlite::SqlitePool;
use std::env;
use anyhow::Result;

pub async fn init_db() -> Result<SqlitePool> {
    // Get database URL from environment or use default
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "data/uptime-kuma.db".to_string());

    // Create database directory if it doesn't exist
    if let Some(parent) = std::path::Path::new(&database_url).parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Create connection pool
    let pool = SqlitePool::connect(&database_url).await?;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(pool)
}

pub async fn close_db(pool: &SqlitePool) -> Result<()> {
    pool.close().await;
    Ok(())
}
