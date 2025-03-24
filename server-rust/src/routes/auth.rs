use axum::{
    extract::State,
    routing::post,
    Router,
    Json,
};
use std::sync::Arc;
use crate::{
    models::user::{CreateUser, LoginUser},
    services::auth::AuthService,
    error::AppError,
};

pub fn auth_routes() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

async fn register(
    State(auth_service): State<Arc<AuthService>>,
    Json(user): Json<CreateUser>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = auth_service.register(user).await?;
    Ok(Json(serde_json::json!({
        "message": "User registered successfully",
        "user": {
            "id": user.id,
            "username": user.username,
            "email": user.email,
        }
    })))
}

async fn login(
    State(auth_service): State<Arc<AuthService>>,
    Json(credentials): Json<LoginUser>,
) -> Result<Json<serde_json::Value>, AppError> {
    let token = auth_service.login(credentials).await?;
    Ok(Json(serde_json::json!({
        "token": token,
        "message": "Login successful"
    })))
}
