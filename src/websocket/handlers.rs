use crate::websocket::{WebSocketManager, WebSocketMessage, WebSocketClient};
use crate::services::{
    monitor::MonitorService,
    heartbeat::HeartbeatService,
    notification::NotificationService,
    status_page::StatusPageService,
    maintenance::MaintenanceService,
    api_key::ApiKeyService,
    docker_host::DockerHostService,
    remote_browser::RemoteBrowserService,
};
use std::sync::Arc;

pub struct WebSocketHandlers {
    ws_manager: Arc<WebSocketManager>,
    monitor_service: Arc<MonitorService>,
    heartbeat_service: Arc<HeartbeatService>,
    notification_service: Arc<NotificationService>,
    status_page_service: Arc<StatusPageService>,
    maintenance_service: Arc<MaintenanceService>,
    api_key_service: Arc<ApiKeyService>,
    docker_host_service: Arc<DockerHostService>,
    remote_browser_service: Arc<RemoteBrowserService>,
}

impl WebSocketHandlers {
    pub fn new(
        ws_manager: Arc<WebSocketManager>,
        monitor_service: Arc<MonitorService>,
        heartbeat_service: Arc<HeartbeatService>,
        notification_service: Arc<NotificationService>,
        status_page_service: Arc<StatusPageService>,
        maintenance_service: Arc<MaintenanceService>,
        api_key_service: Arc<ApiKeyService>,
        docker_host_service: Arc<DockerHostService>,
        remote_browser_service: Arc<RemoteBrowserService>,
    ) -> Self {
        Self {
            ws_manager,
            monitor_service,
            heartbeat_service,
            notification_service,
            status_page_service,
            maintenance_service,
            api_key_service,
            docker_host_service,
            remote_browser_service,
        }
    }

    // Monitor handlers
    pub async fn handle_monitor_update(&self, monitor_id: i64, user_id: i64) {
        if let Ok(monitor) = self.monitor_service.get_monitor(monitor_id, user_id).await {
            self.ws_manager.broadcast_to_user(user_id, WebSocketMessage::MonitorUpdate(monitor)).await;
        }
    }

    pub async fn handle_monitor_delete(&self, monitor_id: i64, user_id: i64) {
        self.ws_manager.broadcast_to_user(user_id, WebSocketMessage::MonitorDelete(monitor_id)).await;
    }

    pub async fn handle_monitor_list(&self, user_id: i64) {
        if let Ok(monitors) = self.monitor_service.get_user_monitors(user_id).await {
            self.ws_manager.broadcast_to_user(user_id, WebSocketMessage::MonitorList(monitors)).await;
        }
    }

    // Heartbeat handlers
    pub async fn handle_heartbeat_update(&self, heartbeat: Heartbeat, user_id: i64) {
        self.ws_manager.broadcast_to_user(user_id, WebSocketMessage::HeartbeatUpdate(heartbeat)).await;
    }

    pub async fn handle_heartbeat_list(&self, monitor_id: i64, user_id: i64) {
        if let Ok(heartbeats) = self.heartbeat_service.get_monitor_heartbeats(monitor_id, user_id).await {
            self.ws_manager.broadcast_to_user(user_id, WebSocketMessage::HeartbeatList(heartbeats)).await;
        }
    }

    // Notification handlers
    pub async fn handle_notification_update(&self, notification_id: i64, user_id: i64) {
        if let Ok(notification) = self.notification_service.get_notification(notification_id, user_id).await {
            self.ws_manager.broadcast_to_user(user_id, WebSocketMessage::NotificationUpdate(notification)).await;
        }
    }

    pub async fn handle_notification_delete(&self, notification_id: i64, user_id: i64) {
        self.ws_manager.broadcast_to_user(user_id, WebSocketMessage::NotificationDelete(notification_id)).await;
    }

    pub async fn handle_notification_list(&self, user_id: i64) {
        if let Ok(notifications) = self.notification_service.get_user_notifications(user_id).await {
            self.ws_manager.broadcast_to_user(user_id, WebSocketMessage::NotificationList(notifications)).await;
        }
    }

    // Status page handlers
    pub async fn handle_status_page_update(&self, status_page_id: i64, user_id: i64) {
        if let Ok(status_page) = self.status_page_service.get_status_page(status_page_id, user_id).await {
            self.ws_manager.broadcast_to_user(user_id, WebSocketMessage::StatusPageUpdate(status_page)).await;
        }
    }

    pub async fn handle_status_page_delete(&self, status_page_id: i64, user_id: i64) {
        self.ws_manager.broadcast_to_user(user_id, WebSocketMessage::StatusPageDelete(status_page_id)).await;
    }

    pub async fn handle_status_page_list(&self, user_id: i64) {
        if let Ok(status_pages) = self.status_page_service.get_user_status_pages(user_id).await {
            self.ws_manager.broadcast_to_user(user_id, WebSocketMessage::StatusPageList(status_pages)).await;
        }
    }

    // Chart data handler
    pub async fn handle_chart_data(&self, monitor_id: i64, user_id: i64) {
        if let Ok(data) = self.heartbeat_service.get_monitor_chart_data(monitor_id, user_id).await {
            self.ws_manager.broadcast_to_user(user_id, WebSocketMessage::ChartData {
                monitor_id,
                data,
            }).await;
        }
    }

    // Maintenance handlers
    pub async fn handle_maintenance_update(&self, maintenance_id: i64, user_id: i64) {
        if let Ok(maintenance) = self.maintenance_service.get_maintenance(maintenance_id, user_id).await {
            self.ws_manager.broadcast_to_user(user_id, WebSocketMessage::MaintenanceUpdate(maintenance)).await;
        }
    }

    pub async fn handle_maintenance_delete(&self, maintenance_id: i64, user_id: i64) {
        self.ws_manager.broadcast_to_user(user_id, WebSocketMessage::MaintenanceDelete(maintenance_id)).await;
    }

    pub async fn handle_maintenance_list(&self, user_id: i64) {
        if let Ok(maintenances) = self.maintenance_service.get_user_maintenances(user_id).await {
            self.ws_manager.broadcast_to_user(user_id, WebSocketMessage::MaintenanceList(maintenances)).await;
        }
    }

    // API key handlers
    pub async fn handle_api_key_update(&self, api_key_id: i64, user_id: i64) {
        if let Ok(api_key) = self.api_key_service.get_api_key(api_key_id, user_id).await {
            self.ws_manager.broadcast_to_user(user_id, WebSocketMessage::ApiKeyUpdate(api_key)).await;
        }
    }

    pub async fn handle_api_key_delete(&self, api_key_id: i64, user_id: i64) {
        self.ws_manager.broadcast_to_user(user_id, WebSocketMessage::ApiKeyDelete(api_key_id)).await;
    }

    pub async fn handle_api_key_list(&self, user_id: i64) {
        if let Ok(api_keys) = self.api_key_service.get_user_api_keys(user_id).await {
            self.ws_manager.broadcast_to_user(user_id, WebSocketMessage::ApiKeyList(api_keys)).await;
        }
    }

    // Docker host handlers
    pub async fn handle_docker_host_update(&self, docker_host_id: i64, user_id: i64) {
        if let Ok(docker_host) = self.docker_host_service.get_docker_host(docker_host_id, user_id).await {
            self.ws_manager.broadcast_to_user(user_id, WebSocketMessage::DockerHostUpdate(docker_host)).await;
        }
    }

    pub async fn handle_docker_host_delete(&self, docker_host_id: i64, user_id: i64) {
        self.ws_manager.broadcast_to_user(user_id, WebSocketMessage::DockerHostDelete(docker_host_id)).await;
    }

    pub async fn handle_docker_host_list(&self, user_id: i64) {
        if let Ok(docker_hosts) = self.docker_host_service.get_user_docker_hosts(user_id).await {
            self.ws_manager.broadcast_to_user(user_id, WebSocketMessage::DockerHostList(docker_hosts)).await;
        }
    }

    // Remote browser handlers
    pub async fn handle_remote_browser_update(&self, browser_id: i64, user_id: i64) {
        if let Ok(browser) = self.remote_browser_service.get_remote_browser(browser_id, user_id).await {
            self.ws_manager.broadcast_to_user(user_id, WebSocketMessage::RemoteBrowserUpdate(browser)).await;
        }
    }

    pub async fn handle_remote_browser_delete(&self, browser_id: i64, user_id: i64) {
        self.ws_manager.broadcast_to_user(user_id, WebSocketMessage::RemoteBrowserDelete(browser_id)).await;
    }

    pub async fn handle_remote_browser_list(&self, user_id: i64) {
        if let Ok(browsers) = self.remote_browser_service.get_user_remote_browsers(user_id).await {
            self.ws_manager.broadcast_to_user(user_id, WebSocketMessage::RemoteBrowserList(browsers)).await;
        }
    }

    // System info handler
    pub async fn handle_system_info(&self, user_id: i64) {
        let info = WebSocketMessage::Info {
            version: env!("CARGO_PKG_VERSION").to_string(),
            uptime: chrono::Utc::now().timestamp() - self.start_time,
            server_time: chrono::Utc::now().timestamp(),
        };
        self.ws_manager.broadcast_to_user(user_id, info).await;
    }
}
