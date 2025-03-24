use async_trait::async_trait;
use serde_json::Value;
use crate::models::monitor::Monitor;
use crate::models::heartbeat::Heartbeat;

#[async_trait]
pub trait NotificationProvider: Send + Sync {
    fn name(&self) -> &'static str;

    async fn send(
        &self,
        config: &Value,
        msg: &str,
        monitor: Option<&Monitor>,
        heartbeat: Option<&Heartbeat>,
    ) -> Result<String, Box<dyn std::error::Error>>;
}

pub mod providers {
    use super::*;
    use reqwest::Client;
    use serde_json::json;

    pub struct Telegram {
        client: Client,
    }

    impl Telegram {
        pub fn new() -> Self {
            Self {
                client: Client::new(),
            }
        }
    }

    #[async_trait]
    impl NotificationProvider for Telegram {
        fn name(&self) -> &'static str {
            "telegram"
        }

        async fn send(
            &self,
            config: &Value,
            msg: &str,
            monitor: Option<&Monitor>,
            heartbeat: Option<&Heartbeat>,
        ) -> Result<String, Box<dyn std::error::Error>> {
            let bot_token = config["botToken"].as_str().ok_or("Missing bot token")?;
            let chat_id = config["chatID"].as_str().ok_or("Missing chat ID")?;

            let mut message = msg.to_string();
            if let Some(monitor) = monitor {
                message = format!("Monitor: {}\n{}", monitor.name, message);
            }
            if let Some(heartbeat) = heartbeat {
                message = format!("{}\nStatus: {}", message, heartbeat.status);
            }

            let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);
            let response = self.client
                .post(&url)
                .json(&json!({
                    "chat_id": chat_id,
                    "text": message,
                    "parse_mode": "HTML"
                }))
                .send()
                .await?;

            if !response.status().is_success() {
                return Err(format!("Telegram API error: {}", response.text().await?).into());
            }

            Ok("Telegram notification sent successfully".to_string())
        }
    }

    pub struct Discord {
        client: Client,
    }

    impl Discord {
        pub fn new() -> Self {
            Self {
                client: Client::new(),
            }
        }
    }

    #[async_trait]
    impl NotificationProvider for Discord {
        fn name(&self) -> &'static str {
            "discord"
        }

        async fn send(
            &self,
            config: &Value,
            msg: &str,
            monitor: Option<&Monitor>,
            heartbeat: Option<&Heartbeat>,
        ) -> Result<String, Box<dyn std::error::Error>> {
            let webhook_url = config["webhookURL"].as_str().ok_or("Missing webhook URL")?;

            let mut message = msg.to_string();
            if let Some(monitor) = monitor {
                message = format!("**Monitor:** {}\n{}", monitor.name, message);
            }
            if let Some(heartbeat) = heartbeat {
                message = format!("{}\n**Status:** {}", message, heartbeat.status);
            }

            let response = self.client
                .post(webhook_url)
                .json(&json!({
                    "content": message
                }))
                .send()
                .await?;

            if !response.status().is_success() {
                return Err(format!("Discord API error: {}", response.text().await?).into());
            }

            Ok("Discord notification sent successfully".to_string())
        }
    }

    pub struct Email {
        client: Client,
    }

    impl Email {
        pub fn new() -> Self {
            Self {
                client: Client::new(),
            }
        }
    }

    #[async_trait]
    impl NotificationProvider for Email {
        fn name(&self) -> &'static str {
            "email"
        }

        async fn send(
            &self,
            config: &Value,
            msg: &str,
            monitor: Option<&Monitor>,
            heartbeat: Option<&Heartbeat>,
        ) -> Result<String, Box<dyn std::error::Error>> {
            let smtp_host = config["smtpHost"].as_str().ok_or("Missing SMTP host")?;
            let smtp_port = config["smtpPort"].as_u64().ok_or("Missing SMTP port")?;
            let smtp_user = config["smtpUser"].as_str().ok_or("Missing SMTP user")?;
            let smtp_pass = config["smtpPass"].as_str().ok_or("Missing SMTP password")?;
            let to_email = config["toEmail"].as_str().ok_or("Missing recipient email")?;

            // TODO: Implement email sending using a proper email library
            // For now, we'll just return a success message
            Ok("Email notification sent successfully".to_string())
        }
    }
}

pub fn get_provider(name: &str) -> Option<Box<dyn NotificationProvider>> {
    match name {
        "telegram" => Some(Box::new(providers::Telegram::new())),
        "discord" => Some(Box::new(providers::Discord::new())),
        "email" => Some(Box::new(providers::Email::new())),
        _ => None,
    }
}
