use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySql, Pool};

// Defining the structure
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Route {
    pub route_id: i32,
    pub origin: String,
    pub destination: String,
    pub distance: f32,
    pub estimated_duration: String,
}

// Implementing the structure
impl Route {
    // Creating a constructor
    pub fn new(
        origin: String,
        destination: String,
        distance: f32,
        estimated_duration: String,
    ) -> Self {
        Self {
            route_id: 0,
            origin,
            destination,
            distance,
            estimated_duration,
        }
    }

    pub async fn find_by_id(pool: &Pool<Mysql>, id: i32) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM routes WHERE route_id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
    }
}
