use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

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

impl Room {
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
