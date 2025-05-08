## Configuration

This project requires configuration for WiFi and API credentials. To set up:

Create a secrets.rs in the config directory at the same level of the config.rs with 

```
pub const WIFI_SSID: &str = "";
pub const WIFI_PASSWORD: &str = "";
pub const CAM_CAPTURE_URL: &str = "";
```

⚠️ Never commit `secrets.rs` to version control!