use bson::Uuid;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Status {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub picture_id: Uuid,
    pub authorised: bool,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
}

impl Status {
    pub fn new(picture_id: Uuid) -> Self {
        Self {
            id: Uuid::new(),
            picture_id,
            authorised: false,
            created_at: Local::now(),
            updated_at: None,
        }
    }
}
