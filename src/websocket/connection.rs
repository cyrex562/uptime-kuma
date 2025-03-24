use std::sync::Arc;
use tokio_tungstenite::{accept_async, WebSocketStream};
use tokio::net::TcpStream;
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use crate::websocket::{WebSocketManager, WebSocketMessage, WebSocketClient};
use crate::websocket::handlers::WebSocketHandlers;
use crate::auth::AuthUser;

pub struct WebSocketConnection {
    ws_manager: Arc<WebSocketManager>,
    handlers: Arc<WebSocketHandlers>,
}

impl WebSocketConnection {
    pub fn new(ws_manager: Arc<WebSocketManager>, handlers: Arc<WebSocketHandlers>) -> Self {
        Self {
            ws_manager,
            handlers,
        }
    }

    pub async fn handle_connection(&self, stream: TcpStream, user: AuthUser) {
        let ws_stream = accept_async(stream).await.expect("Failed to accept WebSocket connection");
        let client_id = self.ws_manager.add_client(user.id, ws_stream).await;

        let mut client = self.ws_manager.get_client(user.id, client_id).await;

        // Send initial data
        self.send_initial_data(&mut client, user.id).await;

        // Handle incoming messages
        while let Some(msg) = client.ws.next().await {
            if let Ok(msg) = msg {
                if let Ok(text) = msg.to_text() {
                    if let Ok(value) = serde_json::from_str::<Value>(text) {
                        self.handle_message(&mut client, value, user.id).await;
                    }
                }
            } else {
                break;
            }
        }

        // Remove client when connection is closed
        self.ws_manager.remove_client(user.id, client_id).await;
    }

    async fn send_initial_data(&self, client: &mut WebSocketClient, user_id: i64) {
        // Send system info
        self.handlers.handle_system_info(user_id).await;

        // Send monitor list
        self.handlers.handle_monitor_list(user_id).await;

        // Send notification list
        self.handlers.handle_notification_list(user_id).await;

        // Send status page list
        self.handlers.handle_status_page_list(user_id).await;

        // Send maintenance list
        self.handlers.handle_maintenance_list(user_id).await;

        // Send API key list
        self.handlers.handle_api_key_list(user_id).await;

        // Send Docker host list
        self.handlers.handle_docker_host_list(user_id).await;

        // Send remote browser list
        self.handlers.handle_remote_browser_list(user_id).await;
    }

    async fn handle_message(&self, client: &mut WebSocketClient, value: Value, user_id: i64) {
        if let Some(action) = value.get("action").and_then(|v| v.as_str()) {
            match action {
                // Monitor actions
                "get_monitor_list" => {
                    self.handlers.handle_monitor_list(user_id).await;
                }
                "get_monitor" => {
                    if let Some(monitor_id) = value.get("monitor_id").and_then(|v| v.as_i64()) {
                        self.handlers.handle_monitor_update(monitor_id, user_id).await;
                    }
                }
                "get_heartbeat_list" => {
                    if let Some(monitor_id) = value.get("monitor_id").and_then(|v| v.as_i64()) {
                        self.handlers.handle_heartbeat_list(monitor_id, user_id).await;
                    }
                }
                "get_chart_data" => {
                    if let Some(monitor_id) = value.get("monitor_id").and_then(|v| v.as_i64()) {
                        self.handlers.handle_chart_data(monitor_id, user_id).await;
                    }
                }

                // Notification actions
                "get_notification_list" => {
                    self.handlers.handle_notification_list(user_id).await;
                }
                "get_notification" => {
                    if let Some(notification_id) = value.get("notification_id").and_then(|v| v.as_i64()) {
                        self.handlers.handle_notification_update(notification_id, user_id).await;
                    }
                }

                // Status page actions
                "get_status_page_list" => {
                    self.handlers.handle_status_page_list(user_id).await;
                }
                "get_status_page" => {
                    if let Some(status_page_id) = value.get("status_page_id").and_then(|v| v.as_i64()) {
                        self.handlers.handle_status_page_update(status_page_id, user_id).await;
                    }
                }

                // Maintenance actions
                "get_maintenance_list" => {
                    self.handlers.handle_maintenance_list(user_id).await;
                }
                "get_maintenance" => {
                    if let Some(maintenance_id) = value.get("maintenance_id").and_then(|v| v.as_i64()) {
                        self.handlers.handle_maintenance_update(maintenance_id, user_id).await;
                    }
                }

                // API key actions
                "get_api_key_list" => {
                    self.handlers.handle_api_key_list(user_id).await;
                }
                "get_api_key" => {
                    if let Some(api_key_id) = value.get("api_key_id").and_then(|v| v.as_i64()) {
                        self.handlers.handle_api_key_update(api_key_id, user_id).await;
                    }
                }

                // Docker host actions
                "get_docker_host_list" => {
                    self.handlers.handle_docker_host_list(user_id).await;
                }
                "get_docker_host" => {
                    if let Some(docker_host_id) = value.get("docker_host_id").and_then(|v| v.as_i64()) {
                        self.handlers.handle_docker_host_update(docker_host_id, user_id).await;
                    }
                }

                // Remote browser actions
                "get_remote_browser_list" => {
                    self.handlers.handle_remote_browser_list(user_id).await;
                }
                "get_remote_browser" => {
                    if let Some(browser_id) = value.get("browser_id").and_then(|v| v.as_i64()) {
                        self.handlers.handle_remote_browser_update(browser_id, user_id).await;
                    }
                }

                // System actions
                "get_system_info" => {
                    self.handlers.handle_system_info(user_id).await;
                }

                _ => {
                    // Unknown action
                    let error = WebSocketMessage::Error(format!("Unknown action: {}", action));
                    if let Err(e) = client.ws.send(serde_json::to_string(&error).unwrap().into()).await {
                        eprintln!("Failed to send error message: {}", e);
                    }
                }
            }
        }
    }
}
