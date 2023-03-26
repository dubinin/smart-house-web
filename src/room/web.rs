use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use sqlx::SqlitePool;

pub use super::{NewRoom, Room};

pub async fn create_room(
    State(pool): State<SqlitePool>,
    Json(new_room): Json<NewRoom>,
) -> Result<Json<Room>, (StatusCode, String)> {
    match new_room.create(&pool).await {
        Ok(id) => Ok(Json(Room {
            id,
            name: new_room.name,
            description: new_room.description,
        })),
        Err(sql_err) => Err((StatusCode::INTERNAL_SERVER_ERROR, sql_err.to_string())),
    }
}

pub async fn all_rooms(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Room>>, (StatusCode, String)> {
    match Room::find_all(&pool).await {
        Ok(rooms) => Ok(Json(rooms)),
        Err(sql_err) => Err((StatusCode::INTERNAL_SERVER_ERROR, sql_err.to_string())),
    }
}

pub async fn delete_room(
    Path(room_id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<StatusCode, (StatusCode, String)> {
    match Room::delete(room_id, &pool).await {
        Ok(true) => Ok(StatusCode::OK),
        Ok(false) => Ok(StatusCode::NOT_FOUND),
        Err(sql_err) => Err((StatusCode::INTERNAL_SERVER_ERROR, sql_err.to_string())),
    }
}
