use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::db::Database;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Notification {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub config: serde_json::Value,
    pub is_default: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub name: String,
    pub type_: String,
    pub is_default: bool,
    #[serde(flatten)]
    pub provider_config: serde_json::Value,
}

impl Notification {
    pub async fn create(db: &Database, user_id: i64, config: NotificationConfig) -> Result<Self, sqlx::Error> {
        let now = chrono::Utc::now();
        let result = sqlx::query_as!(
            Notification,
            r#"
            INSERT INTO notification (user_id, name, config, is_default, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, user_id, name, config, is_default, created_at, updated_at
            "#,
            user_id,
            config.name,
            config.provider_config as _,
            config.is_default,
            now,
            now
        )
        .fetch_one(&db.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_by_id(db: &Database, id: i64, user_id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Notification,
            r#"
            SELECT id, user_id, name, config, is_default, created_at, updated_at
            FROM notification
            WHERE id = $1 AND user_id = $2
            "#,
            id,
            user_id
        )
        .fetch_optional(&db.pool)
        .await
    }

    pub async fn get_by_user(db: &Database, user_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            Notification,
            r#"
            SELECT id, user_id, name, config, is_default, created_at, updated_at
            FROM notification
            WHERE user_id = $1
            ORDER BY name
            "#,
            user_id
        )
        .fetch_all(&db.pool)
        .await
    }

    pub async fn update(db: &Database, id: i64, user_id: i64, config: NotificationConfig) -> Result<Self, sqlx::Error> {
        let now = chrono::Utc::now();
        sqlx::query_as!(
            Notification,
            r#"
            UPDATE notification
            SET name = $1, config = $2, is_default = $3, updated_at = $4
            WHERE id = $5 AND user_id = $6
            RETURNING id, user_id, name, config, is_default, created_at, updated_at
            "#,
            config.name,
            config.provider_config as _,
            config.is_default,
            now,
            id,
            user_id
        )
        .fetch_one(&db.pool)
        .await
    }

    pub async fn delete(db: &Database, id: i64, user_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM notification
            WHERE id = $1 AND user_id = $2
            "#,
            id,
            user_id
        )
        .execute(&db.pool)
        .await?;

        Ok(())
    }

    pub async fn get_monitor_notifications(db: &Database, monitor_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            Notification,
            r#"
            SELECT n.id, n.user_id, n.name, n.config, n.is_default, n.created_at, n.updated_at
            FROM notification n
            JOIN monitor_notification mn ON mn.notification_id = n.id
            WHERE mn.monitor_id = $1
            "#,
            monitor_id
        )
        .fetch_all(&db.pool)
        .await
    }
}
