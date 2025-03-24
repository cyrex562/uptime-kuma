use crate::db::Database;
use crate::models::notification::{Notification, NotificationConfig};
use crate::notifications::{get_provider, NotificationProvider};
use crate::models::monitor::Monitor;
use crate::models::heartbeat::Heartbeat;

pub struct NotificationService {
    db: Database,
}

impl NotificationService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn create_notification(&self, user_id: i64, config: NotificationConfig) -> Result<Notification, Box<dyn std::error::Error>> {
        if get_provider(&config.type_).is_none() {
            return Err("Invalid notification type".into());
        }

        Notification::create(&self.db, user_id, config).await.map_err(|e| e.into())
    }

    pub async fn get_notification(&self, id: i64, user_id: i64) -> Result<Option<Notification>, Box<dyn std::error::Error>> {
        Notification::get_by_id(&self.db, id, user_id).await.map_err(|e| e.into())
    }

    pub async fn get_user_notifications(&self, user_id: i64) -> Result<Vec<Notification>, Box<dyn std::error::Error>> {
        Notification::get_by_user(&self.db, user_id).await.map_err(|e| e.into())
    }

    pub async fn update_notification(&self, id: i64, user_id: i64, config: NotificationConfig) -> Result<Notification, Box<dyn std::error::Error>> {
        if get_provider(&config.type_).is_none() {
            return Err("Invalid notification type".into());
        }

        Notification::update(&self.db, id, user_id, config).await.map_err(|e| e.into())
    }

    pub async fn delete_notification(&self, id: i64, user_id: i64) -> Result<(), Box<dyn std::error::Error>> {
        Notification::delete(&self.db, id, user_id).await.map_err(|e| e.into())
    }

    pub async fn send_notification(
        &self,
        notification: &Notification,
        msg: &str,
        monitor: Option<&Monitor>,
        heartbeat: Option<&Heartbeat>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let provider = get_provider(&notification.config["type"].as_str().ok_or("Missing notification type")?)
            .ok_or("Invalid notification type")?;

        provider.send(&notification.config, msg, monitor, heartbeat).await
    }

    pub async fn test_notification(&self, notification: &Notification) -> Result<String, Box<dyn std::error::Error>> {
        self.send_notification(notification, &format!("Test notification for {}", notification.name), None, None).await
    }

    pub async fn get_monitor_notifications(&self, monitor_id: i64) -> Result<Vec<Notification>, Box<dyn std::error::Error>> {
        Notification::get_monitor_notifications(&self.db, monitor_id).await.map_err(|e| e.into())
    }
}
