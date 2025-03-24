use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

pub mod user;
pub mod monitor;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub email: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Monitor {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub url: String,
    pub type_: String,
    pub interval: i32,
    pub timeout: i32,
    pub status: String,
    pub last_check: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub type_: String,
    pub config: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusPage {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
