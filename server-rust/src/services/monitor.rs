use sqlx::SqlitePool;
use crate::{
    models::monitor::{Monitor, CreateMonitor, UpdateMonitor},
    error::AppError,
};
use reqwest::Client;
use std::time::Duration;

pub struct MonitorService {
    pool: SqlitePool,
    http_client: Client,
}

impl MonitorService {
    pub fn new(pool: SqlitePool) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();
        Self { pool, http_client }
    }

    pub async fn create(&self, user_id: i64, monitor: CreateMonitor) -> Result<Monitor, AppError> {
        let monitor = Monitor::create(&self.pool, user_id, monitor).await?;
        Ok(monitor)
    }

    pub async fn get(&self, id: i64, user_id: i64) -> Result<Monitor, AppError> {
        let monitor = Monitor::find_by_id(&self.pool, id, user_id)
            .await?
            .ok_or(AppError::NotFound)?;
        Ok(monitor)
    }

    pub async fn list(&self, user_id: i64) -> Result<Vec<Monitor>, AppError> {
        let monitors = Monitor::list_by_user(&self.pool, user_id).await?;
        Ok(monitors)
    }

    pub async fn update(
        &self,
        id: i64,
        user_id: i64,
        monitor: UpdateMonitor,
    ) -> Result<Monitor, AppError> {
        let monitor = Monitor::update(&self.pool, id, user_id, monitor)
            .await?
            .ok_or(AppError::NotFound)?;
        Ok(monitor)
    }

    pub async fn delete(&self, id: i64, user_id: i64) -> Result<bool, AppError> {
        let deleted = Monitor::delete(&self.pool, id, user_id).await?;
        Ok(deleted)
    }

    pub async fn check_status(&self, id: i64, user_id: i64) -> Result<(), AppError> {
        let monitor = self.get(id, user_id).await?;
        let start_time = std::time::Instant::now();

        let result = match monitor.type_.as_str() {
            "http" | "https" => self.check_http(&monitor).await,
            "ping" => self.check_ping(&monitor).await,
            _ => Err(AppError::BadRequest("Unsupported monitor type".to_string())),
        };

        let ping = start_time.elapsed().as_millis() as i32;
        let (status, message) = match result {
            Ok(_) => ("up", None),
            Err(e) => ("down", Some(e.to_string())),
        };

        Monitor::update_status(&self.pool, id, status, Some(ping), message.as_deref()).await?;
        Ok(())
    }

    async fn check_http(&self, monitor: &Monitor) -> Result<(), AppError> {
        let response = self
            .http_client
            .get(&monitor.url)
            .timeout(Duration::from_secs(monitor.timeout as u64))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AppError::BadRequest(format!(
                "HTTP status code: {}",
                response.status()
            )));
        }

        Ok(())
    }

    async fn check_ping(&self, monitor: &Monitor) -> Result<(), AppError> {
        // Extract hostname from URL
        let hostname = monitor
            .url
            .replace("http://", "")
            .replace("https://", "")
            .split('/')
            .next()
            .ok_or_else(|| AppError::BadRequest("Invalid URL".to_string()))?;

        // Use the system's ping command
        let output = tokio::process::Command::new("ping")
            .arg("-c")
            .arg("1")
            .arg("-W")
            .arg(monitor.timeout.to_string())
            .arg(hostname)
            .output()
            .await?;

        if !output.status.success() {
            return Err(AppError::BadRequest("Ping failed".to_string()));
        }

        Ok(())
    }
}
