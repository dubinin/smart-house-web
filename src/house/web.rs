use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use sqlx::SqlitePool;

use super::House;

pub async fn get_house(
    State(pool): State<SqlitePool>,
) -> Result<Json<House>, (StatusCode, String)> {
    match House::get(&pool).await {
        Ok(house) => Ok(Json(house)),
        Err(sql_err) => Err((StatusCode::INTERNAL_SERVER_ERROR, sql_err.to_string())),
    }
}
