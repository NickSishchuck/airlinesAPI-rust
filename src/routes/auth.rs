use axum::{
    routing::{get, post, put},
    Router,
};
use crate::{
    db::DbPool,
    handlers::auth,
    middleware::auth::authenticated,
};

pub fn auth_routes() -> Router<DbPool> {
    Router::new()
        // Public routes that don't require authentication
        .route("/register", post(auth::register_email))
        .route("/login", post(auth::login))
        .route("/login-phone", post(auth::login_phone))
        // Protected routes that require authentication
        .merge(protected_routes())
}

// Routes that require authentication
fn protected_routes() -> Router<DbPool> {
    Router::new()
        .route("/me", get(auth::get_me))
        .route("/logout", get(auth::logout))
        .layer(authenticated())
}