use axum::{routing::get, Router, Json};
use serde_json::json;

use crate::db::DbPool;

mod auth;
mod users;
// Add more route modules as they're created
// mod aircraft;
// mod crews;
// mod crew_members;
// mod flights;
// mod flight_seats;
// mod routes;
// mod tickets;

pub fn app_router(pool: DbPool) -> Router {
    Router::new()
        .route("/", get(root_handler))
        .nest("/api/auth", auth::auth_routes())
        .nest("/api/users", users::user_routes())
        // Add more routes as they're created
        // .nest("/api/aircraft", aircraft::aircraft_routes())
        // .nest("/api/crews", crews::crew_routes())
        // .nest("/api/crew-members", crew_members::crew_member_routes())
        // .nest("/api/flights", flights::flight_routes())
        // .nest("/api/flight-seats", flight_seats::flight_seat_routes())
        // .nest("/api/routes", routes::route_routes())
        // .nest("/api/tickets", tickets::ticket_routes())
        .with_state(pool)
}

async fn root_handler() -> Json<serde_json::Value> {
    Json(json!({
        "message": "Welcome to Airline Transportation API",
    }))
}