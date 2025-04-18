use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use crate::{
    auth::create_token,
    db::DbPool,
    error::{AppError, Result},
    middleware::auth::AuthUser,
    models::user::{User, CreateUserDto, LoginDto, PhoneLoginDto, UserRole},
};

// Register a new user with email
pub async fn register_email(
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

    // Create the user
    let user_id = User::create(&pool, user_data).await?;

    // Get created user
    let user = User::find_by_id(&pool, user_id).await?;

    // Create token
    let token = create_token(user.user_id, user.role)?;

    Ok(Json(json!({
        "success": true,
        "token": token,
        "data": user
    })))
}

// Login with email and password
pub async fn login(
    State(pool): State<DbPool>,
    Json(login_data): Json<LoginDto>,
) -> Result<impl IntoResponse> {
    // Validate required fields
    if login_data.email.is_empty() || login_data.password.is_empty() {
        return Err(AppError::ValidationError(
            "Please provide email and password".to_string(),
        ));
    }

    // Find user by email
    let user = User::find_by_email(&pool, &login_data.email).await?;

    // Verify password
    if !User::verify_password(&user, &login_data.password).await? {
        return Err(AppError::AuthError("Invalid credentials".to_string()));
    }

    // Create token
    let token = create_token(user.user_id, user.role)?;

    // Remove password from user object for response
    Ok(Json(json!({
        "success": true,
        "token": token,
        "data": {
            "user_id": user.user_id,
            "email": user.email,
            "role": user.role,
            "first_name": user.first_name,
            "last_name": user.last_name,
            "passport_number": user.passport_number,
            "nationality": user.nationality,
            "date_of_birth": user.date_of_birth,
            "contact_number": user.contact_number,
            "gender": user.gender,
            "created_at": user.created_at,
            "updated_at": user.updated_at,
        }
    })))
}

// Login with phone and password
pub async fn login_phone(
    State(pool): State<DbPool>,
    Json(login_data): Json<PhoneLoginDto>,
) -> Result<impl IntoResponse> {
    // Validate required fields
    if login_data.phone.is_empty() || login_data.password.is_empty() {
        return Err(AppError::ValidationError(
            "Please provide phone and password".to_string(),
        ));
    }

    // Find user by phone
    let user = User::find_by_phone(&pool, &login_data.phone).await?;

    // Verify password
    if !User::verify_password(&user, &login_data.password).await? {
        return Err(AppError::AuthError("Invalid credentials".to_string()));
    }

    // Create token
    let token = create_token(user.user_id, user.role)?;

    // Remove password from user object for response
    Ok(Json(json!({
        "success": true,
        "token": token,
        "data": {
            "user_id": user.user_id,
            "email": user.email,
            "role": user.role,
            "first_name": user.first_name,
            "last_name": user.last_name,
            "passport_number": user.passport_number,
            "nationality": user.nationality,
            "date_of_birth": user.date_of_birth,
            "contact_number": user.contact_number,
            "gender": user.gender,
            "created_at": user.created_at,
            "updated_at": user.updated_at,
        }
    })))
}

// Get current user
pub async fn get_me(
    State(pool): State<DbPool>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse> {
    // Get user data
    let user = User::find_by_id(&pool, auth_user.user_id).await?;

    Ok(Json(json!({
        "success": true,
        "data": user
    })))
}

// Logout (Just a placeholder since JWT is stateless)
pub async fn logout() -> impl IntoResponse {
    Json(json!({
        "success": true,
        "data": {}
    }))
}