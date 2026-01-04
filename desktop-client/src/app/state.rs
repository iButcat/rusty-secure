use reqwest::Client;

use crate::services::RustySecureApiImpl;

#[derive(Debug, Clone)]
pub enum Page {
    Home,
    Settings,
    Login,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub current_page: Page,
    pub loading: bool,
    pub error_message: Option<String>,
    pub user_logged_in: bool,
    pub api_service: RustySecureApiImpl,
    pub token_input: String,
    pub user: Option<crate::models::User>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_page: Page::Login,
            loading: false,
            error_message: None,
            user_logged_in: false,
            api_service: RustySecureApiImpl::new(Client::new(), "http://0.0.0.0:8080".to_string()),
            token_input: String::new(),
            user: None,
        }
    }
}

impl AppState {
    pub fn is_logged_in(&self) -> bool {
        self.user_logged_in
    }

    pub fn set_loading(&mut self, loading: bool) {
        self.loading = loading;
    }

    pub fn set_error(&mut self, error: Option<String>) {
        self.error_message = error;
    }
}
