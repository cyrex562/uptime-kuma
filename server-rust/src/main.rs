use axum::{
    routing::get,
    Router,
    middleware,
};
use tower_http::cors::CorsLayer;
use tokio::net::TcpListener;
use std::sync::Arc;
use sqlx::sqlite::SqlitePool;
use dotenv::dotenv;

mod models;
mod routes;
mod services;
mod error;
mod config;
mod middleware;

use config::database::{init_db, close_db};
use routes::{health::health_check, auth::auth_routes, monitor::monitor_routes};
use services::{auth::AuthService, monitor::MonitorService};
use middleware::auth::auth_middleware;

#[derive(Clone)]
struct AppState {
    db: SqlitePool,
    auth_service: Arc<AuthService>,
    monitor_service: Arc<MonitorService>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();

    // Initialize logging
    tracing_subscriber::fmt::init();

    // Initialize database
    let db = init_db().await?;
    let auth_service = Arc::new(AuthService::new(db.clone()));
    let monitor_service = Arc::new(MonitorService::new(db.clone()));
    let state = Arc::new(AppState {
        db: db.clone(),
        auth_service: auth_service.clone(),
        monitor_service: monitor_service.clone(),
    });

    // Create router
    let app = Router::new()
        .route("/", get(health_check))
        .route("/api/status", get(health_check))
        .nest("/api/auth", auth_routes())
        .nest("/api/monitors", monitor_routes())
        .layer(CorsLayer::permissive())
        .layer(middleware::from_fn(auth_middleware))
        .with_state(state.clone());

    // Start server
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("Server running on http://127.0.0.1:3000");

    // Run the server
    axum::serve(listener, app).await?;

    // Close database connection
    close_db(&db).await?;

    Ok(())
}
