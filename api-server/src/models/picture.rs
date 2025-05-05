use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use bson::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Picture {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
}

impl Picture {
    pub fn new(name: String, url: String) -> Self {
        Self {
            id: Uuid::new(),
            name,
            url,
            created_at: Local::now(),
            updated_at: None,
        }
    }
}