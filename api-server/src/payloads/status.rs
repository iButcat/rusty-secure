use bson::Uuid;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::models::{Picture, Status};
use crate::payloads::picture::PictureResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    pub id: Uuid,
    pub picture: PictureResponse,
    pub authorised: bool,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
}

impl StatusResponse {
    pub fn new(status: Status, picture: Picture) -> Self {
        Self {
            id: status.id,
            picture: PictureResponse::new(picture),
            authorised: status.authorised,
            created_at: status.created_at,
            updated_at: status.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorisedPatchRequest {
    pub authorised: bool,
}
