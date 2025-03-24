use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use crate::{
    services::auth::AuthService,
    error::AppError,
};

pub async fn auth_middleware<B>(
    State(auth_service): State<Arc<AuthService>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // Skip auth for public routes
    if req.uri().path().starts_with("/api/auth") ||
       req.uri().path() == "/" ||
       req.uri().path() == "/api/status" {
        return Ok(next.run(req).await);
    }

    // Get token from Authorization header
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Verify token
    let claims = auth_service
        .verify_token(auth_header)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Add user info to request extensions
    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}
