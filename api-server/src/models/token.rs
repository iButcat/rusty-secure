use bson::Uuid;
use chrono::{DateTime, Duration, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub token_type: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: Option<Duration>,
    pub scopes: Option<Vec<String>>,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
}

impl Token {
    pub fn new(
        token_type: String,
        access_token: String, 
        refresh_token: String,
        expires_at: Option<Duration>, 
        scopes: Option<Vec<String>>,
    ) -> Self {
        Self {
            id: Uuid::new(),
            token_type,
            access_token,
            refresh_token,
            expires_at,
            scopes,
            created_at: Local::now(),
            updated_at: None,
        }
    }
}