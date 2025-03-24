use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Monitor {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub url: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub interval: i32,
    pub timeout: i32,
    pub status: String,
    pub last_check: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMonitor {
    pub name: String,
    pub url: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub interval: Option<i32>,
    pub timeout: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMonitor {
    pub name: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub interval: Option<i32>,
    pub timeout: Option<i32>,
}

impl Monitor {
    pub async fn create(
        pool: &sqlx::SqlitePool,
        user_id: i64,
        monitor: CreateMonitor,
    ) -> Result<Self, sqlx::Error> {
        let result = sqlx::query_as!(
            Monitor,
            r#"
            INSERT INTO monitors (
                user_id, name, url, type, interval, timeout, status
            )
            VALUES (?, ?, ?, ?, ?, ?, 'unknown')
            RETURNING id, user_id, name, url, type, interval, timeout, status, last_check, created_at, updated_at
            "#,
            user_id,
            monitor.name,
            monitor.url,
            monitor.type_,
            monitor.interval.unwrap_or(60),
            monitor.timeout.unwrap_or(30)
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_id(
        pool: &sqlx::SqlitePool,
        id: i64,
        user_id: i64,
    ) -> Result<Option<Self>, sqlx::Error> {
        let result = sqlx::query_as!(
            Monitor,
            r#"
            SELECT id, user_id, name, url, type, interval, timeout, status, last_check, created_at, updated_at
            FROM monitors
            WHERE id = ? AND user_id = ?
            "#,
            id,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    pub async fn list_by_user(
        pool: &sqlx::SqlitePool,
        user_id: i64,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let result = sqlx::query_as!(
            Monitor,
            r#"
            SELECT id, user_id, name, url, type, interval, timeout, status, last_check, created_at, updated_at
            FROM monitors
            WHERE user_id = ?
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(result)
    }

    pub async fn update(
        pool: &sqlx::SqlitePool,
        id: i64,
        user_id: i64,
        monitor: UpdateMonitor,
    ) -> Result<Option<Self>, sqlx::Error> {
        let result = sqlx::query_as!(
            Monitor,
            r#"
            UPDATE monitors
            SET
                name = COALESCE(?, name),
                url = COALESCE(?, url),
                type = COALESCE(?, type),
                interval = COALESCE(?, interval),
                timeout = COALESCE(?, timeout)
            WHERE id = ? AND user_id = ?
            RETURNING id, user_id, name, url, type, interval, timeout, status, last_check, created_at, updated_at
            "#,
            monitor.name,
            monitor.url,
            monitor.type_,
            monitor.interval,
            monitor.timeout,
            id,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    pub async fn delete(
        pool: &sqlx::SqlitePool,
        id: i64,
        user_id: i64,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM monitors
            WHERE id = ? AND user_id = ?
            "#,
            id,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn update_status(
        pool: &sqlx::SqlitePool,
        id: i64,
        status: &str,
        ping: Option<i32>,
        message: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE monitors
            SET status = ?, last_check = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
            status,
            id
        )
        .execute(pool)
        .await?;

        if let Some(ping) = ping {
            sqlx::query!(
                r#"
                INSERT INTO monitor_status_history (monitor_id, status, ping, message)
                VALUES (?, ?, ?, ?)
                "#,
                id,
                status,
                ping,
                message
            )
            .execute(pool)
            .await?;
        }

        Ok(())
    }
}
