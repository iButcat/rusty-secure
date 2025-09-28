use std::env;

// NOTE: I could use references to avoid ownership
// however we don't need critical performance :)
#[derive(Clone)]
pub struct Config {
    pub database_name: String,
    pub bucket_name: String,
    pub mongodb_url: String,
    pub credentials_path: String,
    pub http_server_address: String,
    pub google_auth_client_id: String,
    pub google_auth_client_secret: String,
    pub google_auth_redirect_url: String,
    pub google_auth_scope: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            database_name: Self::value_or_fallback(
                env::var("DATABASE_NAME").ok(), 
                "default".to_string()
            ),
            bucket_name: Self::value_or_fallback(
                env::var("BUCKET_NAME").ok(),
                "default".to_string()
            ),
            mongodb_url: Self::value_or_fallback(
                env::var("MONGODB_URL").ok(),
                "mongodb://localhost:27017".to_string()
            ),
            credentials_path: Self::value_or_panic(
                env::var("CREDENTIALS_PATH").ok()
            ),
            http_server_address: Self::value_or_panic(
                env::var("HTTP_SERVER_ADDRESS").ok()
            ),
            google_auth_client_id: Self::value_or_panic(
                env::var("GOOGLE_AUTH_CLIENT_ID").ok()
            ),
            google_auth_client_secret: Self::value_or_panic(
                env::var("GOOGLE_AUTH_CLIENT_SECRET").ok()
            ),
            google_auth_redirect_url: Self::value_or_fallback(
                env::var("GOOGLE_AUTH_REDIRECT_URL").ok(),
                "http://localhost:8080".to_string()
            ),
            google_auth_scope: Self::value_or_fallback(
                env::var("GOOGLE_AUTH_SCOPE").ok(),
                "openid".to_string()
            )
        }
    }

    // When a value can be set with default like db conn str.
    fn value_or_fallback<T>(value: Option<T>, fallback: T) -> T {
        value.unwrap_or(fallback)
    }

    // When a value is necessary to avoid bugs..
    fn value_or_panic<T> (value: Option<T>) -> T {
        match value {
            Some(val) => val,
            None => panic!("panic: value mustn't be empty")
        }
    }
}