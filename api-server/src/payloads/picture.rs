use bson::Uuid;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::models::Picture;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PictureResponse {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
}

impl PictureResponse {
    pub fn new(picture: Picture) -> Self {
        Self {
            id: picture.id,
            name: picture.name,
            url: picture.url,
            created_at: picture.created_at,
            updated_at: picture.updated_at,
        }
    }
}
