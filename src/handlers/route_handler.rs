use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::models::Route;

// Query parameters for pagination
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

// Create route request body
#[derive(Debug, Deserialize)]
pub struct CreateRouteRequest {
    pub origin: String,
    pub destination: String,
    pub distance: f32,
    pub estimated_duration: String,
}

// Update route request body
#[derive(Debug, Deserialize)]
pub struct UpdateRouteRequest {
    pub origin: Option<String>,
    pub destination: Option<String>,
    pub distance: Option<f32>,
    pub estimated_duration: Option<String>,
}

// Response wrapper
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

// Pagination response wrapper
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub success: bool,
    pub count: usize,
    pub pagination: Pagination,
    pub data: Vec<T>,
}

#[derive(Debug, Serialize)]
pub struct Pagination {
    pub page: i32,
    pub limit: i32,
    pub total_pages: i32,
    pub total_items: i64,
}

// Get all routes with pagination
pub async fn get_routes(
    State(pool): State<MySqlPool>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<Route>>, (StatusCode, Json<serde_json::Value>)> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);

    // Get routes and count
    let routes = match Route::find_all(&pool, page, limit).await {
        Ok(routes) => routes,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "success": false,
                    "error": format!("Database error: {}", e)
                })),
            ));
        }
    };

    let total = match Route::count(&pool).await {
        Ok(count) => count,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "success": false,
                    "error": format!("Database error: {}", e)
                })),
            ));
        }
    };
}
