use reqwest::Client;

use super::RustySecureApi;
use crate::models::User;
use crate::errors::Error;

pub struct RustySecureApiImpl {
    client: Client,
    api_base_url: String,
}

impl RustySecureApiImpl {
    pub fn new(
        client: Client,
        api_base_url: String,
    ) -> Self {
        Self { 
            client,
            api_base_url  
        }
    }
}

impl RustySecureApi for RustySecureApiImpl {
    async fn get_auth_url(&self) -> Result<(String, String), crate::errors::Error> {
        Ok(("".to_string(), "".to_string()))   
    }

    async fn get_user_by_google_id(&self, id: String) -> Result<Option<User>, Error>{
        let response = self.
            client
            .get(self.api_base_url)
            .send()
            .await
            .map_err(|e| Error::ApiError(e.to_string()))?;
    }
}