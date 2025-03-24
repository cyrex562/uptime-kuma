mod connection;
mod handlers;

pub use connection::WebSocketConnection;
pub use handlers::WebSocketHandlers;

use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::WebSocketStream;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::models::monitor::Monitor;
use crate::models::heartbeat::Heartbeat;
use crate::models::notification::Notification;
use crate::models::status_page::StatusPage;
use crate::models::maintenance::Maintenance;
use crate::models::api_key::ApiKey;
use crate::models::docker_host::DockerHost;
use crate::models::remote_browser::RemoteBrowser;

#[derive(Debug, Serialize, Deserialize)]
pub enum WebSocketMessage {
    MonitorUpdate {
        id: i64,
        name: String,
        status: String,
        last_check: DateTime<Utc>,
        uptime: f64,
        response_time: i64,
    },
    MonitorDelete(i64),
    MonitorList(Vec<Monitor>),
    HeartbeatUpdate {
        monitor_id: i64,
        status: String,
        ping: i64,
        time: DateTime<Utc>,
    },
    HeartbeatList {
        monitor_id: i64,
        heartbeats: Vec<Heartbeat>,
    },
    NotificationUpdate {
        id: i64,
        name: String,
        type_: String,
        config: serde_json::Value,
    },
    NotificationDelete(i64),
    NotificationList(Vec<Notification>),
    StatusPageUpdate {
        id: i64,
        name: String,
        slug: String,
        config: serde_json::Value,
    },
    StatusPageDelete(i64),
    StatusPageList(Vec<StatusPage>),
    ChartData {
        monitor_id: i64,
        points: Vec<ChartPoint>,
    },
    MaintenanceUpdate {
        id: i64,
        name: String,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        status: String,
    },
    MaintenanceDelete(i64),
    MaintenanceList(Vec<Maintenance>),
    ApiKeyUpdate {
        id: i64,
        name: String,
        key: String,
        created_at: DateTime<Utc>,
    },
    ApiKeyDelete(i64),
    ApiKeyList(Vec<ApiKey>),
    DockerHostUpdate {
        id: i64,
        name: String,
        url: String,
        status: String,
    },
    DockerHostDelete(i64),
    DockerHostList(Vec<DockerHost>),
    RemoteBrowserUpdate {
        id: i64,
        name: String,
        url: String,
        status: String,
    },
    RemoteBrowserDelete(i64),
    RemoteBrowserList(Vec<RemoteBrowser>),
    SystemInfo {
        version: String,
        uptime: i64,
        memory_usage: f64,
        cpu_usage: f64,
    },
    Error(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChartPoint {
    pub time: DateTime<Utc>,
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Monitor {
    pub id: i64,
    pub name: String,
    pub status: String,
    pub last_check: DateTime<Utc>,
    pub uptime: f64,
    pub response_time: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Heartbeat {
    pub status: String,
    pub ping: i64,
    pub time: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    pub id: i64,
    pub name: String,
    pub type_: String,
    pub config: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusPage {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub config: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Maintenance {
    pub id: i64,
    pub name: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: i64,
    pub name: String,
    pub key: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DockerHost {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteBrowser {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub status: String,
}

pub struct WebSocketClient {
    pub ws: WebSocketStream<tokio::net::TcpStream>,
    pub user_id: i64,
}

pub struct WebSocketManager {
    clients: Arc<RwLock<HashMap<(i64, i64), WebSocketClient>>>,
}

impl WebSocketManager {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_client(&self, user_id: i64, ws: WebSocketStream<tokio::net::TcpStream>) -> i64 {
        let client_id = rand::random::<i64>();
        let mut clients = self.clients.write().await;
        clients.insert((user_id, client_id), WebSocketClient { ws, user_id });
        client_id
    }

    pub async fn remove_client(&self, user_id: i64, client_id: i64) {
        let mut clients = self.clients.write().await;
        clients.remove(&(user_id, client_id));
    }

    pub async fn get_client(&self, user_id: i64, client_id: i64) -> WebSocketClient {
        let clients = self.clients.read().await;
        clients.get(&(user_id, client_id)).unwrap().clone()
    }

    pub async fn broadcast_to_user(&self, user_id: i64, message: WebSocketMessage) {
        let clients = self.clients.read().await;
        let message = serde_json::to_string(&message).unwrap();

        for ((uid, _), client) in clients.iter() {
            if *uid == user_id {
                if let Err(e) = client.ws.send(message.clone().into()).await {
                    eprintln!("Failed to send message to client: {}", e);
                }
            }
        }
    }

    pub async fn broadcast_to_all(&self, message: WebSocketMessage) {
        let clients = self.clients.read().await;
        let message = serde_json::to_string(&message).unwrap();

        for (_, client) in clients.iter() {
            if let Err(e) = client.ws.send(message.clone().into()).await {
                eprintln!("Failed to send message to client: {}", e);
            }
        }
    }
}
