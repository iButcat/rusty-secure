use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct TokenData {
    pub access_token: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GoogleUserInfo {
    #[serde(alias = "sub")]
    pub id: String,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AuthResponse {
    pub token: TokenData,
    pub user_info: GoogleUserInfo,
}
