use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use bson::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub google_id: String,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
}

impl User {
    pub fn new(
        google_id: String, 
        email: String, 
        name: String, 
        picture: Option<String>,
    ) -> Self {
            Self {
                id: Uuid::new(),
                google_id,
                email,
                name,
                picture,
                created_at: Local::now(),
                updated_at: None,
            }
    }
}