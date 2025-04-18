use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use crate::error::{AppError, Result};
use crate::models::user::UserRole;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // Subject (user id)
    pub role: UserRole,   // User role
    pub exp: usize,       // Expiration time (seconds since Unix epoch)
    pub iat: usize,       // Issued at (seconds since Unix epoch)
}

pub fn create_token(user_id: i32, role: UserRole) -> Result<String> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiration = env::var("JWT_EXPIRES_IN").unwrap_or_else(|_| "30d".to_string());

    // Parse expiration time (assuming format like "30d", "1h", etc.)
    let expires_in = parse_duration(&expiration).unwrap_or_else(|| Duration::days(30));

    let now = Utc::now();
    let expires_at = now + expires_in;

    let claims = Claims {
        sub: user_id.to_string(),
        role,
        exp: expires_at.timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
        .map_err(|e| AppError::InternalError(format!("Token creation failed: {}", e)))
}

pub fn verify_token(token: &str) -> Result<Claims> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
        .map(|data| data.claims)
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                AppError::AuthError("Token expired".to_string())
            }
            jsonwebtoken::errors::ErrorKind::InvalidToken => {
                AppError::AuthError("Invalid token".to_string())
            }
            _ => AppError::AuthError(format!("Token validation failed: {}", e)),
        })
}

// Parse duration strings like "30d", "24h", etc.
fn parse_duration(duration_str: &str) -> Option<Duration> {
    let (amount_str, unit) = duration_str.split_at(duration_str.len() - 1);
    let amount = amount_str.parse::<i64>().ok()?;

    match unit {
        "s" => Some(Duration::seconds(amount)),
        "m" => Some(Duration::minutes(amount)),
        "h" => Some(Duration::hours(amount)),
        "d" => Some(Duration::days(amount)),
        _ => None,
    }
}