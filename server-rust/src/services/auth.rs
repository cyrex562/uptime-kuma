use sqlx::SqlitePool;
use crate::models::user::{User, CreateUser, LoginUser};
use crate::error::AppError;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub username: String,
    pub exp: i64,
}

pub struct AuthService {
    pool: SqlitePool,
}

impl AuthService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn register(&self, user: CreateUser) -> Result<User, AppError> {
        // Check if username already exists
        if User::find_by_username(&self.pool, &user.username).await?.is_some() {
            return Err(AppError::UsernameTaken);
        }

        // Create new user
        let user = User::create(&self.pool, user).await?;
        Ok(user)
    }

    pub async fn login(&self, credentials: LoginUser) -> Result<String, AppError> {
        // Find user by username
        let user = User::find_by_username(&self.pool, &credentials.username)
            .await?
            .ok_or(AppError::InvalidCredentials)?;

        // Verify password
        if !user.verify_password(&credentials.password) {
            return Err(AppError::InvalidCredentials);
        }

        // Generate JWT token
        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(24))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user.id,
            username: user.username,
            exp: expiration,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?;

        Ok(token)
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, AppError> {
        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }
}
