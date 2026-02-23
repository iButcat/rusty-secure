use bson::Uuid;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    Esp32Main,
    Esp32Cam,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: Uuid,
    pub user_id: Uuid,
    pub device_type: DeviceType,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}
