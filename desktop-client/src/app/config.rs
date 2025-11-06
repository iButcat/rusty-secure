#[derive(Debug, Clone)]
pub struct Config {
    pub api_base_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_base_url: "http://localhost:8080".to_string(),
        }
    }
}