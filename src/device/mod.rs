use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use self::{socket::SmartSocket, thermometer::SmartThermometer};

pub mod socket;
pub mod thermometer;
pub mod web;

pub trait Device {
    fn power(&self) -> u16;

    fn is_on(&self) -> bool;

    fn is_plugable(&self) -> bool;

    fn is_socket(&self) -> bool;

    fn switch(&mut self);
}

#[derive(Serialize, Deserialize)]
pub struct DeviceReport {
    id: i64,
    info: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    devices: Option<Vec<DeviceReport>>,
}

pub trait DisplayableDevice: Device + std::fmt::Display {}

#[derive(Serialize, Deserialize)]
pub struct NewDatabaseDevice {
    room_id: i64,
    parent_id: Option<i64>,
    device_type: String,
    is_on: bool,
}

impl NewDatabaseDevice {
    async fn create(&self, pool: &SqlitePool) -> Result<i64, sqlx::Error> {
        let id = sqlx::query!(
            r#"INSERT INTO devices (room, parent, type, is_on) VALUES (?1, ?2, ?3, ?4)"#,
            self.room_id,
            self.parent_id,
            self.device_type,
            self.is_on,
        )
        .execute(pool)
        .await?
        .last_insert_rowid();
        Ok(id)
    }
}

#[derive(Serialize, Deserialize)]
pub struct DatabaseDevice {
    id: i64,
    room_id: i64,
    parent_id: Option<i64>,
    device_type: String,
    is_on: bool,
}

impl DatabaseDevice {
    pub async fn find_by_id(
        device_id: i64,
        pool: &SqlitePool,
    ) -> Result<DatabaseDevice, sqlx::Error> {
        let device = sqlx::query_as!(DatabaseDevice, r#"
            SELECT id, room as `room_id!`, parent as `parent_id?`, type as device_type, is_on as `is_on!: bool` 
            FROM devices 
            WHERE id = ?1"#, device_id).fetch_one(pool).await?;
        Ok(device)
    }

    pub async fn find_by_room(
        room_id: i64,
        pool: &SqlitePool,
    ) -> Result<Vec<DatabaseDevice>, sqlx::Error> {
        let devices = sqlx::query_as!(DatabaseDevice, r#"
            SELECT id, room as `room_id!`, parent as `parent_id?`, type as device_type, is_on as `is_on!: bool` 
            FROM devices 
            WHERE room = ?1 OR parent IN (SELECT id FROM devices WHERE room = ?1)"#, room_id)
            .fetch_all(pool)
            .await?;
        Ok(devices)
    }

    pub async fn find_by_parent(
        parent_id: i64,
        pool: &SqlitePool,
    ) -> Result<Vec<DatabaseDevice>, sqlx::Error> {
        let devices = sqlx::query_as!(DatabaseDevice, r#"
            SELECT id, room as `room_id!`, parent as parent_id, type as device_type, is_on as `is_on!: bool` 
            FROM devices 
            WHERE parent = ?1"#, parent_id)
            .fetch_all(pool)
            .await?;
        Ok(devices)
    }

    pub async fn delete(device_id: i64, pool: &SqlitePool) -> Result<bool, sqlx::Error> {
        let affected_rows = sqlx::query!(r#"DELETE FROM devices where id = ?1"#, device_id)
            .execute(pool)
            .await?
            .rows_affected();
        Ok(affected_rows > 0)
    }

    pub fn get_id(&self) -> i64 {
        self.id
    }

    pub fn get_parent_id(&self) -> Option<i64> {
        self.parent_id
    }
}

impl DeviceReport {
    pub fn from(db_device: &DatabaseDevice, childs: Option<&Vec<DatabaseDevice>>) -> DeviceReport {
        let info = match db_device.device_type.as_str() {
            "thermometer" => SmartThermometer::from(db_device).to_string(),
            "socket" => SmartSocket::from(db_device).to_string(),
            _ => String::from("Unhandled device type!"),
        };

        DeviceReport {
            id: db_device.id,
            info,
            devices: childs.map(|value| {
                value
                    .iter()
                    .map(|device| DeviceReport::from(device, None))
                    .collect()
            }),
        }
    }
}
