use axum::{
    routing::{delete, get},
    Router,
};
use sqlx::SqlitePool;
use std::{env, net::SocketAddr};

use smart_house_web::house::web::get_house;
use smart_house_web::room::web::{all_rooms, create_room, delete_room};

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();

    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .expect("Can't connect to database");

    let app = Router::new()
        .route("/", get(get_house))
        .route("/rooms", get(all_rooms).post(create_room))
        .route("/rooms/:room_id", delete(delete_room))
        .with_state(pool);

    let port = std::env::var("PORT")
        .unwrap_or("3000".to_string())
        .parse::<u16>()
        .unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
