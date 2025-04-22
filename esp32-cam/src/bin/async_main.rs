use std::sync::Arc;
use std::sync::Mutex as StdMutex;

use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::Mutex as BlockingMutex;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_time::{Duration, Timer};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspNvsPartition;
use esp_idf_svc::wifi::{ClientConfiguration, Configuration as WifiConfiguration, EspWifi};
use esp_idf_hal::gpio::{PinDriver, Gpio4, Output};
use esp_idf_hal::peripherals::Peripherals;
use log::{info, error};

/*
 * TODO: remove refacto from the name. We switch from the module which was using no_std however
 * the cam module isn't easy to setup so we are using idf services instead. I might write 
 * crate to be able to use the cam from esp-32 board with OV2640 sensor in no_std environment.
 * Also need reorganise the main code since we are simply testing now and check format of the code.
 */
use esp32_cam::cam::camera_controller::CameraController;
use esp32_cam::http::server::CameraHttpServer;
use esp32_cam::config::Config;

use heapless::String;

type SharedFlashPin<'a> = Arc<StdMutex<PinDriver<'a, Gpio4, Output>>>;
type SharedCamera<'a> = Arc<BlockingMutex<CriticalSectionRawMutex, CameraController<'a>>>;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    log::info!("Reached main start");

    let peripherals = Peripherals::take().expect("Failed to take peripherals");
    let pins = peripherals.pins;

    let flash_led_driver = match PinDriver::output(pins.gpio4) { Ok(d) => d, Err(e) => { error!("Flash ERR: {}", e); return; } };
    let flash_led: SharedFlashPin = Arc::new(StdMutex::new(flash_led_driver));
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
    let default_nvs = EspNvsPartition::take_with(false)
        .expect("Failed to create default nvs");
    

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

    wifi.start().expect("Failed to start wifi");
    info!("Wifi started");

    wifi.connect().expect("Failed to connect to wifi");
    info!("Wifi connected");

    let mut ip_info = None;
    for _ in 0..20 {
        if let Ok(info) = wifi.sta_netif().get_ip_info() {
            if info.ip != std::net::Ipv4Addr::new(0, 0, 0, 0) {
                ip_info = Some(info);
                break;
            }
        }
        Timer::after(Duration::from_millis(500)).await;
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
        Ok(cam) => Arc::new(BlockingMutex::new(cam)),
        Err(e) => {
            log::error!("Failed to initialize camera: {:?}", e);
            loop { embassy_time::Timer::after(embassy_time::Duration::from_secs(60)).await; }
        }
    };
    info!("Camera controller initialized.");

    camera_controller.lock(|controller_guard| {
        let sensor = controller_guard.sensor();
        if let Err(e) = sensor.set_brightness(1) { error!("Set Brightness ERR: {}", e); }
        if let Err(e) = sensor.set_contrast(1) { error!("Set Contrast ERR: {}", e); }
        if let Err(e) = sensor.set_saturation(0) { error!("Set Saturation ERR: {}", e); }
        if let Err(e) = sensor.set_quality(10) { error!("Set Quality ERR: {}", e); }
        if let Err(e) = sensor.set_whitebal(true) { error!("Set Whitebal ERR: {}", e); }
    });
    info!("Sensor configured.");

    let camera_clone = camera_controller.clone();
    let flash_clone = flash_led.clone();

    let _http_server = match CameraHttpServer::new(camera_clone, flash_clone, config.api_url) {
        Ok(server) => server,
        Err(e) => {
            log::error!("Failed to create HTTP server: {:?}", e);
            loop { embassy_time::Timer::after(embassy_time::Duration::from_secs(60)).await; }
        }
    };

    log::info!("HTTP server initialized");

    loop {
        log::info!("Server running...");
        embassy_time::Timer::after(embassy_time::Duration::from_secs(60)).await;
    }
}

