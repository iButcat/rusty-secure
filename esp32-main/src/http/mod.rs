pub mod client;
pub use client::*;

use heapless::String;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CamAuthResponse {
    pub authorized: bool,
    pub user_id: String<64>,
    pub user_first_name: String<64>,
    pub user_last_name: String<64>,
    pub reason: String<128>,
}

#[derive(Clone, Debug)]
pub enum HttpMessage {
    RequestCapture,
    AuthResult(CamAuthResponse),
}