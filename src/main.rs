mod notifications;
mod services;
mod routes;

use actix_web::{App, HttpServer};
use services::notification::NotificationService;
use routes::notification;
use crate::websocket::{WebSocketManager, WebSocketMessage};
use crate::websocket::handlers::WebSocketHandlers;
use crate::websocket::connection::WebSocketConnection;
use std::sync::Arc;
use actix_web::web::{self, get};
use actix_web::Request;
use actix_web::body::Body;
use actix_web::upgrade;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::new().await?;
    let notification_service = NotificationService::new(db.clone());

    // Initialize WebSocket manager and handlers
    let ws_manager = Arc::new(WebSocketManager::new());
    let ws_handlers = Arc::new(WebSocketHandlers::new(
        ws_manager.clone(),
        monitor_service.clone(),
        heartbeat_service.clone(),
        notification_service.clone(),
        status_page_service.clone(),
        maintenance_service.clone(),
        api_key_service.clone(),
        docker_host_service.clone(),
        remote_browser_service.clone(),
    ));
    let ws_connection = Arc::new(WebSocketConnection::new(ws_manager.clone(), ws_handlers.clone()));

    // Create WebSocket upgrade handler
    let ws_upgrade = move |req: Request<Body>, user: AuthUser| async move {
        let (response, ws_stream) = upgrade::upgrade(req)?;
        let ws_connection = ws_connection.clone();
        tokio::spawn(async move {
            ws_connection.handle_connection(ws_stream, user).await;
        });
        Ok(response)
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(notification_service.clone()))
            .configure(notification::config)
            .route("/ws", get(ws_upgrade))
            // ... existing routes ...
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
