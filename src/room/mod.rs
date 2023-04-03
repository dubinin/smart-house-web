use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::device::DatabaseDevice;

pub mod web;

#[derive(Deserialize, Debug)]
pub struct NewRoom {
    name: String,
    description: String,
}

impl NewRoom {
    async fn create(&self, pool: &SqlitePool) -> Result<i64, sqlx::Error> {
        let id = sqlx::query!(
            r#"INSERT INTO rooms (name, description) VALUES (?1, ?2)"#,
            self.name,
            self.description
        )
        .execute(pool)
        .await?
        .last_insert_rowid();
        Ok(id)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Room {
    id: i64,
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
pub struct RoomFullInfo {
    room: Room,
    devices: Vec<DatabaseDevice>,
}

impl Room {
    pub async fn find(room_id: i64, pool: &SqlitePool) -> Result<RoomFullInfo, sqlx::Error> {
        let room = sqlx::query_as!(
            Room,
            r#"SELECT id, name, description FROM rooms WHERE id = ?1"#,
            room_id
        )
        .fetch_one(pool)
        .await?;
        let devices = DatabaseDevice::find_by_room(room_id, pool).await?;
        Ok(RoomFullInfo { room, devices })
    }

    pub async fn find_all(pool: &SqlitePool) -> Result<Vec<Room>, sqlx::Error> {
        let rooms = sqlx::query_as!(Room, r#"SELECT id, name, description FROM rooms"#)
            .fetch_all(pool)
            .await?;
        Ok(rooms)
    }

    pub async fn delete(room_id: i64, pool: &SqlitePool) -> Result<bool, sqlx::Error> {
        let affected_rows = sqlx::query!(r#"DELETE FROM rooms where id = ?1"#, room_id)
            .execute(pool)
            .await?
            .rows_affected();
        Ok(affected_rows > 0)
    }
}
