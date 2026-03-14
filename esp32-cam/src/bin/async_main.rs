use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use esp_idf_hal::gpio::{Gpio4, Output, PinDriver};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspNvsPartition;
use esp_idf_svc::wifi::{ClientConfiguration, Configuration as WifiConfiguration, EspWifi};
use log::{error, info};

use esp32_cam::cam::camera_controller::CameraController;
use esp32_cam::config::Config;
use esp32_cam::http::server::CameraHttpServer;

use heapless::String;

type SharedFlashPin<'a> = Arc<Mutex<PinDriver<'a, Gpio4, Output>>>;
type SharedCamera<'a> = Arc<Mutex<CameraController<'a>>>;

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    log::info!("Reached main start");

    let peripherals = Peripherals::take().expect("Failed to take peripherals");
    let pins = peripherals.pins;

    let flash_led_driver = match PinDriver::output(pins.gpio4) {
        Ok(d) => d,
        Err(e) => {
            error!("Flash ERR: {}", e);
            return;
        }
    };
    let flash_led: SharedFlashPin = Arc::new(Mutex::new(flash_led_driver));
    {
        match flash_led.lock() {
            Ok(mut guard) => {
                if let Err(e) = guard.set_low() {
                    error!("Failed to set flash led low: {}", e);
                }
            }
            Err(poisoned) => {
                error!("Flash mutex was poisoned: {}", poisoned);
                return;
            }
        }
    }
    info!("Flashlight LED (GPIO4) configured.");

    let sys_loop = EspSystemEventLoop::take().expect("Failed to take sys loop");
    let default_nvs = EspNvsPartition::take_with(false).expect("Failed to create default nvs");

    let mut wifi = EspWifi::new(peripherals.modem, sys_loop, Some(default_nvs))
        .expect("Failed to create wifi");

    let config: Config = Config::new();
    let ssid = String::<32>::try_from(config.ssid).unwrap();
    let password = String::<64>::try_from(config.password).unwrap();
    let wifi_configuration = WifiConfiguration::Client(ClientConfiguration {
        ssid,
        password,
        ..Default::default()
    });

    wifi.set_configuration(&wifi_configuration)
        .expect("Failed to set wifi configuration");

    if let Err(e) = wifi.start() {
        error!("Failed to start wifi: {:?}", e);
        return;
    }

    if let Err(e) = wifi.connect() {
        error!("Failed to connect wifi: {:?}", e);
        return;
    }

    let mut ip_info = None;
    for _ in 0..20 {
        if let Ok(info) = wifi.sta_netif().get_ip_info() {
            if info.ip != std::net::Ipv4Addr::new(0, 0, 0, 0) {
                ip_info = Some(info);
                break;
            }
        }
        thread::sleep(Duration::from_millis(500));
    }

    if let Some(ip_info) = ip_info {
        info!("IP info: {:?}", ip_info);
    } else {
        error!("Failed to get IP info");
        return;
    }

    let camera_controller: SharedCamera = match CameraController::new(
        pins.gpio32,
        pins.gpio0,
        pins.gpio5,
        pins.gpio18,
        pins.gpio19,
        pins.gpio21,
        pins.gpio36,
        pins.gpio39,
        pins.gpio34,
        pins.gpio35,
        pins.gpio25,
        pins.gpio23,
        pins.gpio22,
        pins.gpio26,
        pins.gpio27,
        esp_idf_sys::camera::pixformat_t_PIXFORMAT_JPEG,
        esp_idf_sys::camera::framesize_t_FRAMESIZE_XGA,
    ) {
        Ok(cam) => Arc::new(Mutex::new(cam)),
        Err(e) => {
            log::error!("Failed to initialize camera: {:?}", e);
            return;
        }
    };
    info!("Camera controller initialized.");

    {
        let controller_guard = camera_controller.lock().unwrap();
        let sensor = controller_guard.sensor();
        if let Err(e) = sensor.set_brightness(1) {
            error!("Set Brightness ERR: {}", e);
        }
        if let Err(e) = sensor.set_contrast(1) {
            error!("Set Contrast ERR: {}", e);
        }
        if let Err(e) = sensor.set_saturation(0) {
            error!("Set Saturation ERR: {}", e);
        }
        if let Err(e) = sensor.set_quality(10) {
            error!("Set Quality ERR: {}", e);
        }
        if let Err(e) = sensor.set_whitebal(true) {
            error!("Set Whitebal ERR: {}", e);
        }
    }
    info!("Sensor configured.");

    let camera_clone = camera_controller.clone();
    let flash_clone = flash_led.clone();

    let _http_server = match CameraHttpServer::new(camera_clone, flash_clone, config.api_url) {
        Ok(server) => server,
        Err(e) => {
            log::error!("Failed to create HTTP server: {:?}", e);
            return;
        }
    };

    log::info!("HTTP server initialized");

    loop {
        log::info!("Server running...");
        thread::sleep(Duration::from_secs(60));
    }
}
