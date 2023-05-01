pub mod web;

use std::collections::HashMap;

use crate::{
    device::{DatabaseDevice, DeviceReport},
    room::{web::Room, RoomReport},
};

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Serialize, Deserialize)]
pub struct House {
    description: String,
    rooms: Vec<RoomReport>,
}

impl House {
    async fn get(pool: &SqlitePool) -> Result<House, sqlx::Error> {
        let description = std::env::var("SMART_HOUSE_DESCRIPTION").unwrap_or(String::new());
        Ok(House {
            description,
            rooms: House::get_rooms_reports(pool).await?,
        })
    }

    async fn get_rooms_reports(pool: &SqlitePool) -> Result<Vec<RoomReport>, sqlx::Error> {
        let mut reports: Vec<RoomReport> = vec![];

        for room in Room::find_all(pool).await? {
            let room_id = room.id;
            reports.push(RoomReport {
                room,
                devices: House::room_devices(room_id, pool).await?,
            });
        }

        Result::Ok(reports)
    }

    async fn room_devices(
        room_id: i64,
        pool: &SqlitePool,
    ) -> Result<Vec<DeviceReport>, sqlx::Error> {
        // Нужно получить все устройства в комнате и по каждому устройству сформировать отчет.
        let devices = DatabaseDevice::find_by_room(room_id, pool).await?;

        let by_parent = devices
            .into_iter()
            .map(|device| -> (i64, DatabaseDevice) {
                (device.get_parent_id().unwrap_or(-1), device)
            })
            .fold(HashMap::new(), |mut map, (parent, device)| {
                map.entry(parent).or_insert_with(Vec::new).push(device);
                map
            });

        if let Some(parents) = by_parent.get(&-1) {
            Result::Ok(
                parents
                    .iter()
                    .map(|device| -> DeviceReport {
                        let childs = by_parent.get(&device.get_id());
                        DeviceReport::from(device, childs)
                    })
                    .collect(),
            )
        } else {
            Result::Ok(vec![])
        }
    }
}
