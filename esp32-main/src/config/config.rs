use crate::config::secrets;

pub struct Config {
    pub cam_capture_url: &'static str,
    pub ssid: &'static str,
    pub password: &'static str,
}

impl Config {
    pub fn new() -> Self {
        Self {
            cam_capture_url: secrets::CAM_CAPTURE_URL,
            ssid: secrets::WIFI_SSID,
            password: secrets::WIFI_PASSWORD,
        }
    }
}
