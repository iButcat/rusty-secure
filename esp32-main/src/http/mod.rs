pub mod client;
pub use client::*;

use heapless::String;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub enum HttpMessage {
    RequestCapture,
    StatusResult(CamStatusResponse),
    RequestFailed(ClientError),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CamPictureResponse {
    pub id: String<36>,
    pub name: String<64>,
    pub url: String<128>,
    pub created_at: String<64>,
    #[serde(default)]
    pub updated_at: Option<String<64>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CamStatusResponse {
    pub id: String<36>,
    pub picture: CamPictureResponse,
    pub authorised: bool,
    pub created_at: String<64>,
    #[serde(default)]
    pub updated_at: Option<String<64>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AuthUpdatePayload {
    pub id: String<36>,
    pub authorised: bool,
}
