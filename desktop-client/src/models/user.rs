use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
}

impl User {
    pub fn new(
        id: String,
        email: String,
        name: String,
        picture: Option<String>,
        created_at: DateTime<Local>,
        updated_at: Option<DateTime<Local>>,
    ) -> Self {
        User {
            id,
            email,
            name,
            picture,
            created_at,
            updated_at,
        }
    }
}
