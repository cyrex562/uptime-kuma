use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub email: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

impl User {
    pub async fn create(
        pool: &sqlx::SqlitePool,
        user: CreateUser,
    ) -> Result<Self, sqlx::Error> {
        let password_hash = hash(user.password.as_bytes(), DEFAULT_COST).unwrap();

        let result = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, password_hash, email)
            VALUES (?, ?, ?)
            RETURNING id, username, password_hash, email, created_at, updated_at
            "#,
            user.username,
            password_hash,
            user.email
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_username(
        pool: &sqlx::SqlitePool,
        username: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        let result = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, password_hash, email, created_at, updated_at
            FROM users
            WHERE username = ?
            "#,
            username
        )
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    pub fn verify_password(&self, password: &str) -> bool {
        verify(password.as_bytes(), &self.password_hash).unwrap_or(false)
    }
}
