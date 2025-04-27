use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Not sure if we should have those here,
// have a DTO layer in between the database and the API
#[derive(Debug, Serialize, Deserialize)]
pub struct Authorisation {
    pub id: Uuid,
    pub picture_id: Uuid,
    pub authorised: bool,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl Authorisation {
    pub fn new(picture_id: uuid::Uuid, authorised: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            picture_id,
            authorised,
            created_at: Local::now(),
            updated_at: Local::now(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Picture {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl Picture {
    pub fn new(name: String, url: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            url,
            created_at: Local::now(),
            updated_at: Local::now(),
        }
    }
}