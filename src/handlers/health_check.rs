use axum::{http::StatusCode, Json};
use serde_json::json;
use sqlx::MySqlPool;

pub async fn health_check(
    pool: axum::extract::State<MySqlPool>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Test database connection
    match sqlx::query("SELECT 1").execute(&*pool).await {
        Ok(_) => Ok(Json(json!({
            "status": "ok",
            "message": "Service is healthy",
            "database": "connected"
        }))),
        Err(_) => {
            // Database connection failed
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}