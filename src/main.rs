// src/main.rs
mod config;
mod db;
mod handlers;
mod logging;

use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    logging::setup_logging();

    // Load configuration
    let config = config::Config::from_env().expect("Failed to load configuration");

    info!("Starting application with configuration: {:?}", config);

    // Create database connection pool
    let pool = db::create_pool(&config.database_url)
        .await
        .expect("Failed to create database connection pool");

    // Test connection
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .expect("Failed to connect to database");

    info!("Successfully connected to database");

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(handlers::health_check::health_check))
        .with_state(pool);

    // Run it with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    info!("Listening on {}", addr);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}