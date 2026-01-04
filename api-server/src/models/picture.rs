use bson::Uuid;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Picture {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub url: String,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
}

impl Picture {
    pub fn new(user_id: Uuid, name: String, url: String) -> Self {
        Self {
            id: Uuid::new(),
            user_id,
            name,
            url,
            created_at: Local::now(),
            updated_at: None,
        }
    }
}
