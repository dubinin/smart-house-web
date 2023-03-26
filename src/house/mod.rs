pub mod web;

use crate::room::web::Room;

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Serialize, Deserialize)]
pub struct House {
    description: String,
    rooms: Vec<Room>,
}

impl House {
    async fn get(pool: &SqlitePool) -> Result<House, sqlx::Error> {
        let description = std::env::var("SMART_HOUSE_DESCRIPTION").unwrap_or(String::new());
        Ok(House {
            description,
            rooms: Room::find_all(pool).await?,
        })
    }
}
