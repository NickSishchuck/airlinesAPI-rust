use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::db::{DbPool, map_db_error};
use crate::error::{AppError, Result};

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub user_id: i32,
    pub email: Option<String>,
    pub role: UserRole,
    pub first_name: String,
    pub last_name: String,
    pub passport_number: Option<String>,
    pub nationality: Option<String>,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub contact_number: Option<String>,
    pub gender: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct UserWithPassword {
    pub user_id: i32,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: UserRole,
    pub first_name: String,
    pub last_name: String,
    pub passport_number: Option<String>,
    pub nationality: Option<String>,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub contact_number: Option<String>,
    pub gender: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ENUM", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Worker,
    User,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserDto {
    pub email: Option<String>,
    pub password: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub role: Option<UserRole>,
    pub passport_number: Option<String>,
    pub nationality: Option<String>,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub contact_number: Option<String>,
    pub gender: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserDto {
    pub email: Option<String>,
    pub password: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub passport_number: Option<String>,
    pub nationality: Option<String>,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub contact_number: Option<String>,
    pub gender: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct PhoneLoginDto {
    pub phone: String,
    pub password: String,
}

impl User {
    pub async fn find_all(
        pool: &DbPool,
        page: i64,
        limit: i64
    ) -> Result<(Vec<User>, i64)> {
        let offset = (page - 1) * limit;

        let users = sqlx::query_as::<_, User>(
            "SELECT user_id, email, role, first_name, last_name, passport_number,
                    nationality, date_of_birth, contact_number, gender, created_at, updated_at
             FROM users
             ORDER BY last_name, first_name
             LIMIT ? OFFSET ?"
        )
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
            .map_err(map_db_error)?;

        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
            .fetch_one(pool)
            .await
            .map_err(map_db_error)?;

        Ok((users, count))
    }

    pub async fn find_by_id(pool: &DbPool, id: i32) -> Result<User> {
        sqlx::query_as::<_, User>(
            "SELECT user_id, email, role, first_name, last_name, passport_number,
                    nationality, date_of_birth, contact_number, gender, created_at, updated_at
             FROM users
             WHERE user_id = ?"
        )
            .bind(id)
            .fetch_one(pool)
            .await
            .map_err(map_db_error)
    }

    pub async fn find_by_email(pool: &DbPool, email: &str) -> Result<UserWithPassword> {
        sqlx::query_as::<_, UserWithPassword>(
            "SELECT user_id, email, password, role, first_name, last_name, passport_number,
                    nationality, date_of_birth, contact_number, gender, created_at, updated_at
             FROM users
             WHERE email = ?"
        )
            .bind(email)
            .fetch_one(pool)
            .await
            .map_err(map_db_error)
    }

    pub async fn find_by_phone(pool: &DbPool, phone: &str) -> Result<UserWithPassword> {
        sqlx::query_as::<_, UserWithPassword>(
            "SELECT user_id, email, password, role, first_name, last_name, passport_number,
                    nationality, date_of_birth, contact_number, gender, created_at, updated_at
             FROM users
             WHERE contact_number = ?"
        )
            .bind(phone)
            .fetch_one(pool)
            .await
            .map_err(map_db_error)
    }

    pub async fn check_email_exists(pool: &DbPool, email: &str, exclude_id: Option<i32>) -> Result<bool> {
        let query = match exclude_id {
            Some(id) => {
                sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE email = ? AND user_id != ?")
                    .bind(email)
                    .bind(id)
                    .fetch_one(pool)
                    .await
            },
            None => {
                sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE email = ?")
                    .bind(email)
                    .fetch_one(pool)
                    .await
            }
        };

        let count: i64 = query.map_err(map_db_error)?;
        Ok(count > 0)
    }

    pub async fn check_passport_exists(pool: &DbPool, passport: &str, exclude_id: Option<i32>) -> Result<bool> {
        let query = match exclude_id {
            Some(id) => {
                sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE passport_number = ? AND user_id != ?")
                    .bind(passport)
                    .bind(id)
                    .fetch_one(pool)
                    .await
            },
            None => {
                sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE passport_number = ?")
                    .bind(passport)
                    .fetch_one(pool)
                    .await
            }
        };

        let count: i64 = query.map_err(map_db_error)?;
        Ok(count > 0)
    }

    pub async fn create(pool: &DbPool, user_data: CreateUserDto) -> Result<i32> {
        // Hash password if provided
        let hashed_password = match user_data.password {
            Some(pass) => Some(hash(pass, DEFAULT_COST).map_err(|e| {
                AppError::InternalError(format!("Password hashing failed: {}", e))
            })?),
            None => None,
        };

        // Use transaction to ensure atomicity
        let mut tx = pool.begin().await.map_err(map_db_error)?;

        let role = user_data.role.unwrap_or(UserRole::User);

        let result = sqlx::query(
            "INSERT INTO users (
                email, password, role, first_name, last_name,
                passport_number, nationality, date_of_birth,
                contact_number, gender
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
            .bind(&user_data.email)
            .bind(&hashed_password)
            .bind(&role)
            .bind(&user_data.first_name)
            .bind(&user_data.last_name)
            .bind(&user_data.passport_number)
            .bind(&user_data.nationality)
            .bind(&user_data.date_of_birth)
            .bind(&user_data.contact_number)
            .bind(&user_data.gender)
            .execute(&mut *tx)
            .await
            .map_err(map_db_error)?;

        tx.commit().await.map_err(map_db_error)?;

        Ok(result.last_insert_id() as i32)
    }

    pub async fn update(pool: &DbPool, id: i32, user_data: UpdateUserDto) -> Result<bool> {
        // Hash password if provided
        let hashed_password = match user_data.password {
            Some(pass) => Some(hash(pass, DEFAULT_COST).map_err(|e| {
                AppError::InternalError(format!("Password hashing failed: {}", e))
            })?),
            None => None,
        };

        let mut tx = pool.begin().await.map_err(map_db_error)?;

        // Build the SET part of the query dynamically
        let mut set_clauses = Vec::new();

        if user_data.email.is_some() { set_clauses.push("email = ?"); }
        if hashed_password.is_some() { set_clauses.push("password = ?"); }
        if user_data.first_name.is_some() { set_clauses.push("first_name = ?"); }
        if user_data.last_name.is_some() { set_clauses.push("last_name = ?"); }
        if user_data.passport_number.is_some() { set_clauses.push("passport_number = ?"); }
        if user_data.nationality.is_some() { set_clauses.push("nationality = ?"); }
        if user_data.date_of_birth.is_some() { set_clauses.push("date_of_birth = ?"); }
        if user_data.contact_number.is_some() { set_clauses.push("contact_number = ?"); }
        if user_data.gender.is_some() { set_clauses.push("gender = ?"); }

        // If no parameters were provided, return early
        if set_clauses.is_empty() {
            return Ok(false);
        }

        // Build the full query
        let query = format!(
            "UPDATE users SET {} WHERE user_id = ?",
            set_clauses.join(", ")
        );

        // Start with a query builder
        let mut query_builder = sqlx::query(&query);

        // Bind all the parameters in order
        if let Some(email) = &user_data.email {
            query_builder = query_builder.bind(email);
        }
        if let Some(password) = &hashed_password {
            query_builder = query_builder.bind(password);
        }
        if let Some(first_name) = &user_data.first_name {
            query_builder = query_builder.bind(first_name);
        }
        if let Some(last_name) = &user_data.last_name {
            query_builder = query_builder.bind(last_name);
        }
        if let Some(passport) = &user_data.passport_number {
            query_builder = query_builder.bind(passport);
        }
        if let Some(nationality) = &user_data.nationality {
            query_builder = query_builder.bind(nationality);
        }
        if let Some(date_of_birth) = &user_data.date_of_birth {
            query_builder = query_builder.bind(date_of_birth);
        }
        if let Some(contact_number) = &user_data.contact_number {
            query_builder = query_builder.bind(contact_number);
        }
        if let Some(gender) = &user_data.gender {
            query_builder = query_builder.bind(gender);
        }

        // Finally bind the ID
        query_builder = query_builder.bind(id);

        let result = query_builder
            .execute(&mut *tx)
            .await
            .map_err(map_db_error)?;

        tx.commit().await.map_err(map_db_error)?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete(pool: &DbPool, id: i32) -> Result<bool> {
        // Check if user has tickets first
        let ticket_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tickets WHERE user_id = ?")
            .bind(id)
            .fetch_one(pool)
            .await
            .map_err(map_db_error)?;

        if ticket_count > 0 {
            return Err(AppError::ConflictError("Cannot delete user with existing tickets".to_string()));
        }

        let result = sqlx::query("DELETE FROM users WHERE user_id = ?")
            .bind(id)
            .execute(pool)
            .await
            .map_err(map_db_error)?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn verify_password(user: &UserWithPassword, password: &str) -> Result<bool> {
        let stored_password = user.password.as_ref().ok_or_else(|| {
            AppError::AuthError("No password set for this user".to_string())
        })?;

        verify(password, stored_password)
            .map_err(|e| AppError::InternalError(format!("Password verification failed: {}", e)))
    }
}