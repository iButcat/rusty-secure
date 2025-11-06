

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
    pub username: String,
    pub password: String,
    pub user_logged_in: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_page: Page::Login,
            loading: false, 
            error_message: None,
            username: String::new(),
            password: String::new(),
            user_logged_in: false,
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