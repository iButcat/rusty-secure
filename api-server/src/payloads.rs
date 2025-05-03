use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use bson::Uuid;

use crate::models::{Picture, Status};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PictureResponse {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>
}

impl PictureResponse {
    pub fn new(picture: Picture) -> Self {
        Self {
            id: picture.id,
            name: picture.name,
            url: picture.url,
            created_at: picture.created_at,
            updated_at: picture.updated_at
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    pub id: Uuid,
    pub picture: PictureResponse,
    pub authorised: bool,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>
}

impl StatusResponse {
    pub fn new(status: Status, picture: Picture) -> Self {
        Self {
            id: status.id,
            picture: PictureResponse::new(picture),
            authorised: status.authorised,
            created_at: status.created_at,
            updated_at: status.updated_at
        }
    }
}