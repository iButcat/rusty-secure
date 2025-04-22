use embassy_net::Stack;
use esp_wifi::wifi::WifiController;
use embassy_time::{Duration, Timer};
use log::info;

use crate::config::Config;

#[embassy_executor::task]
pub async fn wifi_connection(
    controller: &'static mut WifiController<'static>,
    stack: &'static Stack<'static>,
    config: &'static Config,
) {
    info!("Starting WiFi connection process");
    
    for retry in 0..5 {
        match controller.start_async().await {
            Ok(_) => {
                info!("WiFi controller started successfully");
                break;
            },
            Err(e) => {
                info!("Failed to start WiFi controller (attempt {}/5): {:?}", retry + 1, e);
                
                if retry == 4 {
                    info!("Failed to start WiFi controller after 5 attempts");
                    return;
                }
                
                Timer::after(Duration::from_millis(2000 * (1 << retry))).await;
            }
        }
    }
    
    info!("Attempting to connect to SSID: {}", config.ssid);
    match controller.connect_async().await {
        Ok(_) => {
            info!("WiFi connected successfully to SSID: {}", config.ssid);
        },
        Err(e) => {
            info!("Failed to connect to SSID: {}: {:?}", config.ssid, e);
            return;
        }
    }
    
    info!("Waiting for IP address...");
    for _ in 0..20 {
        if let Some(cfg) = stack.config_v4() {
            info!("Acquired IP address: {}", cfg.address);
            info!("WiFi network stack initialized.");
            info!("WiFi connected successfully!");
            return;
        }
        Timer::after(Duration::from_millis(100)).await;
    }
    
    info!("Failed to acquire IP address after timeout");
}