use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{
    db::DbPool,
    error::{AppError, Result},
    models::user::{User, CreateUserDto, UpdateUserDto},
};

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

// Get all users with pagination
pub async fn get_users(
    State(pool): State<DbPool>,
    Query(params): Query<PaginationParams>,
) -> Result<impl IntoResponse> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);

    if page < 1 || limit < 1 {
        return Err(AppError::ValidationError("Page and limit must be positive".to_string()));
    }

    let (users, total) = User::find_all(&pool, page, limit).await?;

    Ok(Json(json!({
        "success": true,
        "count": users.len(),
        "pagination": {
            "page": page,
            "limit": limit,
            "totalPages": (total + limit - 1) / limit,
            "totalItems": total
        },
        "data": users
    })))
}

// Get user by ID
pub async fn get_user(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse> {
    let user = User::find_by_id(&pool, id).await?;

    Ok(Json(json!({
        "success": true,
        "data": user
    })))
}

// Create a new user
pub async fn create_user(
    State(pool): State<DbPool>,
    Json(user_data): Json<CreateUserDto>,
) -> Result<impl IntoResponse> {
    // Validate required fields
    if user_data.first_name.is_empty() || user_data.email.is_none() || user_data.password.is_none() {
        return Err(AppError::ValidationError(
            "Please provide name, email and password".to_string(),
        ));
    }

    // Check if email already exists
    if let Some(email) = &user_data.email {
        if User::check_email_exists(&pool, email, None).await? {
            return Err(AppError::ConflictError("Email already in use".to_string()));
        }
    }

    // Check if passport number already exists (if provided)
    if let Some(passport) = &user_data.passport_number {
        if User::check_passport_exists(&pool, passport, None).await? {
            return Err(AppError::ConflictError("Passport number already in use".to_string()));
        }
    }

    // Create the user
    let user_id = User::create(&pool, user_data).await?;

    // Get created user
    let user = User::find_by_id(&pool, user_id).await?;

    Ok(Json(json!({
        "success": true,
        "data": user
    })))
}

// Update a user
pub async fn update_user(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(user_data): Json<UpdateUserDto>,
) -> Result<impl IntoResponse> {
    // Check if user exists
    let _ = User::find_by_id(&pool, id).await?;

    // Check if email is taken (if updating email)
    if let Some(email) = &user_data.email {
        if User::check_email_exists(&pool, email, Some(id)).await? {
            return Err(AppError::ConflictError("Email already in use".to_string()));
        }
    }

    // Check if passport is taken (if updating passport)
    if let Some(passport) = &user_data.passport_number {
        if User::check_passport_exists(&pool, passport, Some(id)).await? {
            return Err(AppError::ConflictError("Passport number already in use".to_string()));
        }
    }

    // Update the user
    User::update(&pool, id, user_data).await?;

    // Get updated user
    let user = User::find_by_id(&pool, id).await?;

    Ok(Json(json!({
        "success": true,
        "data": user
    })))
}

// Delete a user
pub async fn delete_user(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse> {
    // Check if user exists
    let _ = User::find_by_id(&pool, id).await?;

    // Delete the user
    User::delete(&pool, id).await?;

    Ok(Json(json!({
        "success": true,
        "data": {}
    })))
}