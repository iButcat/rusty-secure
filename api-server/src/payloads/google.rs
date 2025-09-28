use serde::{Deserialize, Serialize};

use crate::models::Token;

#[derive(Deserialize, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub name: String, 
    pub picture: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: Token,
    pub user_info: UserInfo
}

#[derive(Deserialize)]
pub struct OAuthCallback {
    pub state: String,
    pub code: String
}