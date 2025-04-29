Ultimate Guide to Creating Routes and Models in Rust with SQLx and Axum
This comprehensive guide will walk you through the process of creating database models and API routes for your Airline Transportation API using Rust with SQLx for database interactions and Axum for handling HTTP requests.
Table of Contents

Architecture Overview
Setting Up SQLx Models
Creating Route Handlers with Axum
Practical Examples
SQLx Schema Verification and Migrations
Best Practices and Tips

Architecture Overview
Your application follows a layered architecture:
src/
├── main.rs              // Application entry point
├── config.rs            // Configuration handling
├── db.rs                // Database connection pool
├── models/              // Data models with SQLx
│   ├── mod.rs           // Re-exports all models
│   ├── model1.rs        // Individual model implementations
│   └── ...
├── handlers/            // API request handlers
│   ├── mod.rs           // Re-exports all handlers
│   ├── handler1.rs      // Individual handler implementations
│   └── ...
├── routes/              // API route definitions
│   ├── mod.rs           // Combines all routes
│   ├── route1.rs        // Individual route modules
│   └── ...
├── middleware/          // Middleware components
│   ├── mod.rs
│   ├── auth.rs          // Authentication middleware
│   └── ...
├── error.rs             // Error handling
└── logging.rs           // Logging setup
Flow of a Request

Client sends a request to a specific endpoint
Axum router directs the request to the appropriate handler
Middleware processes the request (authentication, logging, etc.)
Handler receives the request and extracts parameters/body
handler interacts with models to perform database operations
models execute sqlx queries and return results
handler processes the results and builds a response
response is sent back to the client

setting up sqlx models
models represent your database tables and provide methods for interacting with them.
model structure
each model typically includes:

struct definition: represents a row in the database table
associated functions: static methods for operations like find, count, etc.
methods: instance methods for operations on a specific record
related structs: for complex query results or dtos

step-by-step guide to creating a model

create a new file in the models directory (e.g., src/models/route.rs)
define the model struct with sqlx and serde support
implement database operations as methods
add the model to mod.rs to expose it

example model implementation
here's how to create a model for the route table:
```rust
// src/models/route.rs
use chrono::{datetime, utc};
use serde::{deserialize, serialize};
use sqlx::{fromrow, mysql, pool};

// 1. Define the struct that maps to the database table
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Route {
    pub route_id: Option<i32>,
    pub origin: String,
    pub destination: String,
    pub distance: f32,
    pub estimated_duration: String, // TIME type in MySQL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

// 2. Implement constructor and methods
impl Route {
    // Constructor
    pub fn new(origin: String, destination: String, distance: f32, duration: String) -> Self {
        Self {
            route_id: None,
            origin,
            destination,
            distance,
            estimated_duration: duration,
            created_at: None,
            updated_at: None,
        }
    }

    // 3. Database operations - Find by ID
    pub async fn find_by_id(pool: &Pool<MySql>, id: i32) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM routes WHERE route_id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    // Find all with pagination
    pub async fn find_all(
        pool: &Pool<MySql>,
        page: i32,
        limit: i32
    ) -> Result<Vec<Self>, sqlx::Error> {
        let offset = (page - 1) * limit;

        sqlx::query_as::<_, Self>(
            "SELECT * FROM routes ORDER BY origin, destination LIMIT ? OFFSET ?"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    }

    // Insert new record
    pub async fn insert(&self, pool: &Pool<MySql>) -> Result<i32, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            INSERT INTO routes (origin, destination, distance, estimated_duration)
            VALUES (?, ?, ?, ?)
            "#,
            self.origin,
            self.destination,
            self.distance,
            self.estimated_duration
        )
        .execute(pool)
        .await?;

        Ok(result.last_insert_id() as i32)
    }

    // Update existing record
    pub async fn update(&self, pool: &Pool<MySql>) -> Result<bool, sqlx::Error> {
        let route_id = match self.route_id {
            Some(id) => id,
            None => return Err(sqlx::Error::RowNotFound),
        };

        let result = sqlx::query!(
            r#"
            UPDATE routes
            SET origin = ?, destination = ?, distance = ?, estimated_duration = ?
            WHERE route_id = ?
            "#,
            self.origin,
            self.destination,
            self.distance,
            self.estimated_duration,
            route_id
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    // Delete record
    pub async fn delete(pool: &Pool<MySql>, id: i32) -> Result<bool, sqlx::Error> {
        // First check if route has any associated flights
        let flight_count = sqlx::query!(
            "SELECT COUNT(*) as count FROM flights WHERE route_id = ?",
            id
        )
        .fetch_one(pool)
        .await?
        .count;

        if flight_count > 0 {
            // Route has associated flights, cannot delete
            return Ok(false);
        }

        let result = sqlx::query!("DELETE FROM routes WHERE route_id = ?", id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    // Count total records
    pub async fn count(pool: &Pool<MySql>) -> Result<i64, sqlx::Error> {
        let count = sqlx::query!("SELECT COUNT(*) as count FROM routes")
            .fetch_one(pool)
            .await?
            .count;

        Ok(count)
    }
}
```
SQLx Type Conversions
SQLx maps SQL types to Rust types:
SQL Type Rust Type
INT      | i32
BIGINT   | i64
FLOAT    | f32
DOUBLE   | f64
VARCHAR  | String
TEXT     | String
DATE     | chrono::NaiveDate
TIME     | chrono::NaiveTime or String
DATETIME | chrono::DateTime<Utc>
TIMESTAMP| chrono::DateTime<Utc>
ENUM     | Custom Rust enum with #[sqlx(type_name = "enum")]
BOOLEAN  | bool
JSON     | serde_json::Value or custom struct


Working with Enums
For MySQL ENUM types, define a corresponding Rust enum:
```Rust
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "enum", rename_all = "lowercase")]
pub enum FlightStatus {
    Scheduled,
    Boarding,
    Departed,
    Arrived,
    Delayed,
    Canceled,
}
```

Creating Route Handlers with Axum
After creating your models, you need route handlers to expose API endpoints.
Structure for Route Modules

Create Handler Functions: Implement request handlers
Define Routes: Build an Axum router with your handlers
Apply Middleware: Add authentication, logging, etc.

Step-by-Step Guide to Creating Routes

Create a new file in the handlers directory (e.g., src/handlers/route_handler.rs)
Implement handler functions that use your models
Create a routes file in the routes directory (e.g., src/routes/route_routes.rs)
Define the router using your handlers
Add the router to your main app in main.rs

Example Handler Implementation
```Rust
// src/handlers/route_handler.rs
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

    // Create response
    let response = PaginatedResponse {
        success: true,
        count: routes.len(),
        pagination: Pagination {
            page,
            limit,
            total_pages: ((total as f64) / (limit as f64)).ceil() as i32,
            total_items: total,
        },
        data: routes,
    };

    Ok(Json(response))
}

// Get route by ID
pub async fn get_route(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Route>>, (StatusCode, Json<serde_json::Value>)> {
    match Route::find_by_id(&pool, id).await {
        Ok(Some(route)) => Ok(Json(ApiResponse {
            success: true,
            data: route,
        })),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "success": false,
                "error": format!("Route not found with id {}", id)
            })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "success": false,
                "error": format!("Database error: {}", e)
            })),
        )),
    }
}

// Create a new route
pub async fn create_route(
    State(pool): State<MySqlPool>,
    Json(payload): Json<CreateRouteRequest>,
) -> Result<Json<ApiResponse<Route>>, (StatusCode, Json<serde_json::Value>)> {
    //
```
