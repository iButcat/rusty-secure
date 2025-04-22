use crate::config::secrets;

pub struct Config {
    pub api_url: &'static str,
    pub ssid: &'static str,
    pub password: &'static str,
}

impl Config {
    pub fn new() -> Self {
        Self {
            api_url: secrets::API_URL,
            ssid: secrets::WIFI_SSID,
            password: secrets::WIFI_PASSWORD,
        }
    }
}
