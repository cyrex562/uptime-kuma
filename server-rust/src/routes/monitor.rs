use axum::{
    extract::{State, Path},
    routing::{get, post, put, delete},
    Router,
    Json,
};
use std::sync::Arc;
use crate::{
    models::monitor::{CreateMonitor, UpdateMonitor},
    services::monitor::MonitorService,
    error::AppError,
    middleware::auth::Claims,
};

pub fn monitor_routes() -> Router {
    Router::new()
        .route("/", get(list_monitors))
        .route("/", post(create_monitor))
        .route("/:id", get(get_monitor))
        .route("/:id", put(update_monitor))
        .route("/:id", delete(delete_monitor))
        .route("/:id/check", post(check_monitor))
}

async fn list_monitors(
    State(monitor_service): State<Arc<MonitorService>>,
    claims: Claims,
) -> Result<Json<serde_json::Value>, AppError> {
    let monitors = monitor_service.list(claims.sub).await?;
    Ok(Json(serde_json::json!({
        "monitors": monitors
    })))
}

async fn create_monitor(
    State(monitor_service): State<Arc<MonitorService>>,
    claims: Claims,
    Json(monitor): Json<CreateMonitor>,
) -> Result<Json<serde_json::Value>, AppError> {
    let monitor = monitor_service.create(claims.sub, monitor).await?;
    Ok(Json(serde_json::json!({
        "message": "Monitor created successfully",
        "monitor": monitor
    })))
}

async fn get_monitor(
    State(monitor_service): State<Arc<MonitorService>>,
    claims: Claims,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let monitor = monitor_service.get(id, claims.sub).await?;
    Ok(Json(serde_json::json!({
        "monitor": monitor
    })))
}

async fn update_monitor(
    State(monitor_service): State<Arc<MonitorService>>,
    claims: Claims,
    Path(id): Path<i64>,
    Json(monitor): Json<UpdateMonitor>,
) -> Result<Json<serde_json::Value>, AppError> {
    let monitor = monitor_service.update(id, claims.sub, monitor).await?;
    Ok(Json(serde_json::json!({
        "message": "Monitor updated successfully",
        "monitor": monitor
    })))
}

async fn delete_monitor(
    State(monitor_service): State<Arc<MonitorService>>,
    claims: Claims,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let deleted = monitor_service.delete(id, claims.sub).await?;
    if deleted {
        Ok(Json(serde_json::json!({
            "message": "Monitor deleted successfully"
        })))
    } else {
        Err(AppError::NotFound)
    }
}

async fn check_monitor(
    State(monitor_service): State<Arc<MonitorService>>,
    claims: Claims,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    monitor_service.check_status(id, claims.sub).await?;
    Ok(Json(serde_json::json!({
        "message": "Monitor status check completed"
    })))
}
