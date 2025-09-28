use serde::{Deserialize, Serialize};
use bson::Uuid;

use crate::models::User;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
       pub id: Uuid,
       pub email: String,
       pub name: String, 
       pub picture: Option<String>,
}

impl UserResponse {
    pub fn new(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            name: user.name,
            picture: user.picture,
        }
    }
}