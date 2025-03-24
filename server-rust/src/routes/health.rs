use axum::{
    routing::get,
    Router,
    Json,
};
use serde_json::json;

pub fn health_routes() -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/api/status", get(health_check))
}

pub async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}
