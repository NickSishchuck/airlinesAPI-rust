use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::{
    db::DbPool,
    handlers::user,
    middleware::auth::admin_only,
};

pub fn user_routes() -> Router<DbPool> {
    Router::new()
        .route("/", get(user::get_users).post(user::create_user))
        .route("/:id", get(user::get_user).put(user::update_user).delete(user::delete_user))
        .route_layer(axum::middleware::from_fn(admin_only()))
}