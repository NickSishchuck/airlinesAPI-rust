use std::future::{ready, Ready};
use axum::{
    extract::{FromRef, FromRequestParts, State},
    http::{Request, StatusCode},
    middleware::Next,
    response::{Response, IntoResponse},
};
use async_trait::async_trait;
use crate::{
    auth::{verify_token, Claims},
    db::DbPool,
    error::{AppError, Result},
    models::user::{User, UserRole},
};

// Extractor for authenticated users
pub struct AuthUser {
    pub user_id: i32,
    pub role: UserRole,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    DbPool: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self> {
        // Extract the token from the Authorization header
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|header| {
                if header.starts_with("Bearer ") {
                    Some(header[7..].to_string())
                } else {
                    None
                }
            })
            .ok_or_else(|| AppError::AuthError("Missing or invalid Authorization header".to_string()))?;

        // Verify the token
        let claims = verify_token(&auth_header)?;

        let user_id = claims.sub.parse::<i32>()
            .map_err(|_| AppError::AuthError("Invalid user ID in token".to_string()))?;

        // Create the AuthUser
        Ok(AuthUser {
            user_id,
            role: claims.role,
        })
    }
}

// Middleware for role-based authorization
pub async fn require_auth<Body>(
    mut req: Request<Body>,
    next: Next,
    allowed_roles: &'static [UserRole],
) -> Result<Response>
where
    Body: Send + 'static,
{
    // Extract the AuthUser from the request
    // We need to convert the request into parts and state
    let (mut parts, body) = req.into_parts();
    let auth_user = AuthUser::from_request_parts(&mut parts, &(DbPool)).await?;

    // Check if the user's role is allowed
    if !allowed_roles.contains(&auth_user.role) {
        return Err(AppError::AuthzError(format!(
            "User role '{:?}' is not authorized to access this route",
            auth_user.role
        )));
    }

    // Reconstruct the request and continue
    let mut req = Request::from_parts(parts, body);

    // Store the authenticated user in request extensions for later use
    req.extensions_mut().insert(auth_user);

    Ok(next.run(req).await)
}

// Helper functions for common role combinations
pub fn admin_only<B>() -> axum::middleware::from_fn_with_state::FromFnWithState<(), impl Fn(Request<B>, Next<B>) -> Result<Response, AppError> + Clone, B>
where
    B: Send + 'static + 'static,
{
    axum::middleware::from_fn_with_state(
        (),
        |req, next| require_auth(req, next, &[UserRole::Admin])
    )
}

pub fn staff_only<B>() -> axum::middleware::from_fn_with_state::FromFnWithState<(), impl Fn(Request<B>, Next<B>) -> Result<Response, AppError> + Clone, B>
where
    B: Send + 'static + 'static,
{
    axum::middleware::from_fn_with_state(
        (),
        |req, next| require_auth(req, next, &[UserRole::Admin, UserRole::Worker])
    )
}

pub fn authenticated<B>() -> axum::middleware::from_fn_with_state::FromFnWithState<(), impl Fn(Request<B>, Next<B>) -> Result<Response, AppError> + Clone, B>
where
    B: Send + 'static + 'static,
{
    axum::middleware::from_fn_with_state(
        (),
        |req, next| require_auth(req, next, &[UserRole::Admin, UserRole::Worker, UserRole::User])
    )
}