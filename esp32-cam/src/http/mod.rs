pub mod server;
pub mod client;

use serde::{Deserialize, Serialize}; 

#[derive(Serialize, Deserialize, Debug)]
pub struct AnalysisResponse {
    authorized: bool,
    user_id: String,
    user_first_name: String,
    user_last_name: String,
    reason: String,
}