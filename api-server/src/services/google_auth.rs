use async_trait::async_trait;
use chrono::TimeDelta;
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, RevocationUrl,
    Scope, TokenResponse, TokenUrl,
};

use super::GoogleAuthService;
use crate::errors::Error;
use crate::models::Token;
use crate::payloads::UserInfo;

pub struct GoogleAuthServiceImpl {
    client_id: String,
    client_secret: String,
    redirect_url: String,
    scope: String,
    auth_url: String,
    token_url: String,
    revocation_url: String,
    user_info_url: String,
}

impl GoogleAuthServiceImpl {
    pub fn new(
        client_id: String,
        client_secret: String,
        redirect_url: String,
        scope: String,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            redirect_url,
            scope,
            auth_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
            token_url: "https://www.googleapis.com/oauth2/v3/token".to_string(),
            revocation_url: "https://oauth2.googleapis.com/revoke".to_string(),
            user_info_url: "https://www.googleapis.com/oauth2/v3/userinfo".to_string(),
        }
    }
}

#[async_trait]
impl GoogleAuthService for GoogleAuthServiceImpl {
    async fn get_authorisation_url(
        &self,
        response_type: Option<String>,
    ) -> Result<(String, String), Error> {
        let auth_url = AuthUrl::new(self.auth_url.clone()).map_err(|e| Error::Parse(e.to_string()));

        let token_url =
            TokenUrl::new(self.token_url.clone()).map_err(|e| Error::Parse(e.to_string()));

        let client = BasicClient::new(ClientId::new(self.client_id.clone()))
            .set_client_secret(ClientSecret::new(self.client_secret.clone()))
            .set_auth_uri(auth_url?)
            .set_token_uri(token_url?)
            .set_redirect_uri(
                RedirectUrl::new(self.redirect_url.clone()).expect("Invalid redirect URL"),
            )
            .set_revocation_url(
                RevocationUrl::new(self.revocation_url.clone())
                    .expect("Invalid revocation endpoints URL"),
            );

        let random_token = CsrfToken::new_random();
        let final_token = if let Some(value) = response_type {
            CsrfToken::new(format!("{}:{}", random_token.secret(), value))
        } else {
            random_token
        };

        let (authorize_url, crsf_state) = client
            .authorize_url(|| final_token)
            .add_scopes(
                self.scope
                    .split_whitespace()
                    .map(|s| Scope::new(s.to_string())),
            )
            .url();

        Ok((authorize_url.to_string(), crsf_state.secret().to_string()))
    }

    async fn exchange_code_for_token(
        &self,
        code: String,
        _state: String,
    ) -> Result<(UserInfo, Token), Error> {
        let auth_url = AuthUrl::new(self.auth_url.clone());
        let token_url =
            TokenUrl::new(self.token_url.clone()).map_err(|e| Error::Parse(e.to_string()));

        let client = BasicClient::new(ClientId::new(self.client_id.clone()))
            .set_client_secret(ClientSecret::new(self.client_secret.clone()))
            .set_auth_uri(auth_url.map_err(|e| Error::Parse(e.to_string()))?)
            .set_token_uri(token_url.map_err(|e| Error::Parse(e.to_string()))?)
            .set_redirect_uri(
                RedirectUrl::new(self.redirect_url.clone()).expect("Invalid redirect URL"),
            );

        let http_client = reqwest::Client::new();

        let token_response = client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(&http_client)
            .await
            .map_err(|e| Error::Internal(e.to_string()))?;

        let access_token = token_response.access_token().secret();
        let refresh_token = token_response
            .refresh_token()
            .map(|token| token.secret().to_string())
            .unwrap_or_default();
        let token_type = "Bearer".to_string();
        let expires_at = token_response
            .expires_in()
            .and_then(|duration| TimeDelta::from_std(duration).ok());
        let scopes = token_response
            .scopes()
            .map(|scopes| {
                scopes
                    .iter()
                    .map(|scope| scope.to_string())
                    .collect::<Vec<String>>()
            })
            .unwrap_or_default();

        let user_info_response: reqwest::Response = http_client
            .get(self.user_info_url.clone())
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| Error::Internal(e.to_string()))?;

        let user_info: UserInfo = user_info_response
            .json()
            .await
            .map_err(|e| Error::JSONUnmarshall(e.to_string()))?;

        let token = Token::new(
            token_type,
            access_token.to_string(),
            refresh_token,
            expires_at,
            Some(scopes),
        );

        Ok((user_info, token))
    }
}
