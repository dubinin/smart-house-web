use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use sqlx::SqlitePool;

use super::{DatabaseDevice, NewDatabaseDevice};

pub async fn create_device(
    State(pool): State<SqlitePool>,
    Json(new_device): Json<NewDatabaseDevice>,
) -> Result<Json<DatabaseDevice>, (StatusCode, String)> {
    match new_device.create(&pool).await {
        Ok(id) => Ok(Json(DatabaseDevice {
            id,
            room_id: new_device.room_id,
            device_type: new_device.device_type,
            parent_id: new_device.parent_id,
            is_on: new_device.is_on,
        })),
        Err(sql_err) => Err((StatusCode::INTERNAL_SERVER_ERROR, sql_err.to_string())),
    }
}

pub async fn get_device(
    Path(device_id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<Json<DatabaseDevice>, (StatusCode, String)> {
    match DatabaseDevice::find_by_id(device_id, &pool).await {
        Ok(device) => Ok(Json(device)),
        Err(sql_err) => Err((StatusCode::INTERNAL_SERVER_ERROR, sql_err.to_string())),
    }
}

pub async fn delete_device(
    Path(device_id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<StatusCode, (StatusCode, String)> {
    match DatabaseDevice::delete(device_id, &pool).await {
        Ok(true) => Ok(StatusCode::OK),
        Ok(false) => Ok(StatusCode::NOT_FOUND),
        Err(sql_err) => Err((StatusCode::INTERNAL_SERVER_ERROR, sql_err.to_string())),
    }
}
