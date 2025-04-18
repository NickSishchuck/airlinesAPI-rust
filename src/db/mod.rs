use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::env;
use std::time::Duration;
use anyhow::Result;

pub type DbPool = MySqlPool;

pub async fn establish_connection() -> Result<DbPool> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await?;

    // Verify connection
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await?;

    tracing::info!("Database connection established");
    Ok(pool)
}

// Helper function to map database errors to app errors
pub fn map_db_error(err: sqlx::Error) -> crate::error::AppError {
    use sqlx::error::ErrorKind;
    use crate::error::AppError;

    match err {
        sqlx::Error::Database(db_err) => {
            // MySQL specific error codes
            let code = db_err.code().unwrap_or_default().to_string();

            // Check for common error codes
            if code == "23000" || code == "1062" {
                // Duplicate entry violation
                AppError::ConflictError("Duplicate entry violation".to_string())
            } else if code == "23503" || code == "1452" {
                // Foreign key constraint violation
                AppError::ValidationError("Foreign key constraint violation".to_string())
            } else {
                AppError::DatabaseError(sqlx::Error::Database(db_err))
            }
        },
        sqlx::Error::RowNotFound => {
            AppError::NotFoundError("Resource not found".to_string())
        },
        _ => AppError::DatabaseError(err),
    }
}