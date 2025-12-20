use std::collections::HashMap;

use reqwest::Client;

use super::RustySecureApi;
use crate::errors::Error;
use crate::models::User;

pub struct RustySecureApiImpl {
    client: Client,
    api_base_url: String,
}

impl RustySecureApiImpl {
    pub fn new(client: Client, api_base_url: String) -> Self {
        Self {
            client,
            api_base_url,
        }
    }
}

impl RustySecureApi for RustySecureApiImpl {
    async fn get_auth_url(&self) -> Result<(String, String), crate::errors::Error> {
        let response = self
            .client
            .get(format!("{}/api/auth/url", self.api_base_url.clone()))
            .send()
            .await
            .map_err(|e| Error::ApiError(e.to_string()))?;

        if response.status() != 200 {
            return Err(Error::ApiError("Unexpected status code".to_string()));
        }

        let auth_url = response
            .text()
            .await
            .map_err(|e| Error::ApiError(e.to_string()))?;
        let state = response
            .headers()
            .get("X-State")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        Ok((auth_url, state))
    }

    async fn get_user_by_google_id(&self, id: String) -> Result<Option<User>, Error> {
        let response = self
            .client
            .get(format!("{}/user/{}", self.api_base_url.clone(), id))
            .send()
            .await
            .map_err(|e| Error::ApiError(e.to_string()))?;

        if response.status() != 200 {
            return Err(Error::ApiError("Unexpected status code".to_string()));
        }

        let user_data = response
            .json::<User>()
            .await
            .map_err(|e| Error::ApiError(e.to_string()))?;

        Ok(Some(user_data))
    }
}
