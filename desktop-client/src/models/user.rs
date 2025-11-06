use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>
}