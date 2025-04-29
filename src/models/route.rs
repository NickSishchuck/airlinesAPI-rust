use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySql, MySqlPool, Pool};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Route {
    pub route_id: i32,
    pub origin: String,
    pub destination: String,
    pub distance: f32,
    pub estimated_duration: chrono::NaiveTime,
}

impl Route {
    pub fn new(
        origin: String,
        destination: String,
        distance: f32,
        estimated_duration: chrono::NaiveTime,
    ) -> Self {
        Self {
            route_id: 0,
            origin,
            destination,
            distance,
            estimated_duration,
        }
    }

    pub async fn find_by_id(pool: &Pool<MySql>, id: i32) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM routes WHERE route_id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_all(
        pool: &MySqlPool,
        page: i32,
        limit: i32,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let offset = (page - 1) * limit;
        sqlx::query_as::<_, Self>("SELECT * FROM routes LIMIT ? OFFSET ?")
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
    }

    pub async fn count(pool: &MySqlPool) -> Result<i64, sqlx::Error> {
        let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM routes")
            .fetch_one(pool)
            .await?;
        Ok(count)
    }
}
