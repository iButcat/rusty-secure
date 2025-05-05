pub mod server;
pub mod client;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PictureResponse {
    id: String,
    name: String, 
    url: String,
    created_at: DateTime<Local>,
    updated_at: Option<DateTime<Local>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusResponse {
    id: String,
    picture: PictureResponse,
    authorised: bool,
    created_at: DateTime<Local>,
    updated_at: Option<DateTime<Local>>
}