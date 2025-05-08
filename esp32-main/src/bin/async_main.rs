#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_net::Stack;
use embassy_sync::channel::Channel;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Input, Level, Output, InputConfig, OutputConfig};
use esp_hal::i2c::master::{I2c, Config as I2cConfig, BusTimeout};
use esp_hal::timer::timg::TimerGroup;
use esp_hal::rng::Rng;
use esp_wifi::wifi::{
    ClientConfiguration, 
    Configuration, 
    Interfaces, 
    WifiController
};
use static_cell::StaticCell;
use log::info;
use esp_backtrace as _;
use embassy_net::StackResources;

use esp32_main::sensor::{SensorMessage, UltrasonicSensor};
use esp32_main::light::{Led, LedMessage};
use esp32_main::display::{LcdDisplay, DisplayMessage};
use esp32_main::http::HttpMessage;
use esp32_main::config::Config as ProjectConfig;
use esp32_main::tasks::{
    display_task, http_camera_task, http_server_task, led_task, net_runner, sensor_task, wifi_connection
};
use esp_hal::time::Rate;

extern crate alloc;

static DISPLAY_CHANNEL: StaticCell<Channel<CriticalSectionRawMutex, DisplayMessage, 2>> = StaticCell::new();
static LED_CHANNEL: StaticCell<Channel<CriticalSectionRawMutex, LedMessage, 1>> = StaticCell::new();
static SENSOR_CHANNEL: StaticCell<Channel<CriticalSectionRawMutex, SensorMessage, 1>> = StaticCell::new();
static PROJECT_CONFIG: StaticCell<ProjectConfig> = StaticCell::new();
static STACK_INIT: StaticCell<Stack<'static>> = StaticCell::new();
static STACK_RESOURCES: StaticCell<StackResources<4>> = StaticCell::new();
static HTTP_CHANNEL: StaticCell<Channel<CriticalSectionRawMutex, HttpMessage, 1>> = StaticCell::new();
static CONTROLLER: StaticCell<Interfaces<'static>> = StaticCell::new();
static WIFI_CONTROLLER: StaticCell<WifiController<'static>> = StaticCell::new();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // NOTE: This is necessary to avoid "StoreProhibited" exceptions when using BLE
    // The ESP32 BLE stack consumes ~64KB of RAM, so we place heap in the dram2 section
    // See: https://github.com/esp-rs/esp-hal/discussions/3188
    esp_alloc::heap_allocator!(#[link_section = ".dram2_uninit"] size: 64 * 1024);
    
    esp_println::logger::init_logger_from_env();

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    info!("Embassy initialized!");
    
    // --- LED Task ---
    let led = Output::new(
        peripherals.GPIO2, 
        Level::Low,
        OutputConfig::default()
    );
    
    let led_driver = Led::new(led);
    let led_channel = LED_CHANNEL.init(Channel::new());
    let _led_sender = led_channel.sender();
    let led_receiver = led_channel.receiver();

    // spawner.spawn(led_task(led_driver, led_receiver)).unwrap(); // Temporarily disable

    // --- Display Task ---
    let i2c_config = I2cConfig::default()
    .with_frequency(Rate::from_hz(100_000))
    .with_timeout(BusTimeout::Maximum);
    
    let scl = peripherals.GPIO22;
    let sda = peripherals.GPIO21;

    let i2c = I2c::new(
        peripherals.I2C0,
        i2c_config,
    ).unwrap().with_scl(scl).with_sda(sda);
    
    let lcd = LcdDisplay::new(i2c);
    let display_channel = DISPLAY_CHANNEL.init(Channel::new());
    let display_sender = display_channel.sender();
    let display_receiver = display_channel.receiver();

    spawner.spawn(display_task(lcd, display_receiver)).unwrap();

    // --- Sensor Task ---
    let trigger = Output::new(
        peripherals.GPIO5, 
        Level::Low,
        OutputConfig::default()
    );
    
    let echo = Input::new(
        peripherals.GPIO19, 
        InputConfig::default()
    );
    
    let sensor = UltrasonicSensor::new(trigger, echo);
    let sensor_channel = SENSOR_CHANNEL.init(Channel::new());
    let _sensor_sender = sensor_channel.sender();
    let sensor_receiver = sensor_channel.receiver();

    spawner.spawn(sensor_task(sensor, sensor_receiver, display_sender)).unwrap();

    let project_config = PROJECT_CONFIG.init(ProjectConfig::new());

    Timer::after(Duration::from_millis(2000)).await;

    let mut ssid = heapless::String::<32>::new();
    ssid.push_str(project_config.ssid).unwrap();
    
    let mut password = heapless::String::<64>::new();
    password.push_str(project_config.password).unwrap();
    
    let client_config = Configuration::Client(ClientConfiguration {
        ssid,
        password,
        ..Default::default()
    });
    info!("Client config prepared: {:?}", client_config);
    
    Timer::after(Duration::from_millis(3000)).await;
    
    info!("Initializing WiFi...");
    let timer1 = TimerGroup::new(peripherals.TIMG0);
    let rng = Rng::new(peripherals.RNG);
    
    let wifi_init = match esp_wifi::init(timer1.timer0, rng, peripherals.RADIO_CLK) {
        Ok(i) => {
            info!("WiFi initialization successful");
            i
        },
        Err(e) => {
            info!("WiFi initialization error: {:?}", e);
            panic!("Failed to initialize WiFi");
        }
    };

    Timer::after(Duration::from_millis(1000)).await;

    static WIFI_INIT: StaticCell<esp_wifi::EspWifiController<'static>> = StaticCell::new();
    let wifi_init = WIFI_INIT.init(wifi_init);

    info!("Creating WiFi controller...");
    let (wifi_controller, mut interfaces) = match esp_wifi::wifi::new(wifi_init, peripherals.WIFI) {
        Ok((c, i)) => {
            info!("WiFi controller created successfully");
            (c, i)
        },
        Err(e) => {
            info!("WiFi controller creation error: {:?}", e);
            panic!("Failed to create WiFi controller");
        }
    };
    
    Timer::after(Duration::from_millis(2000)).await;
    
    let wifi_controller_ref = WIFI_CONTROLLER.init(wifi_controller);

    let interfaces_ref = CONTROLLER.init(interfaces);
    
    info!("Setting WiFi configuration...");
    match wifi_controller_ref.set_configuration(&client_config) {
        Ok(_) => info!("Configuration set successfully"),
        Err(e) => {
            info!("Error setting configuration: {:?}", e);
        }
    }

    let mac_address = interfaces_ref.sta.mac_address();

    let mac_u64: u64 = ((mac_address[0] as u64) << 40)
        | ((mac_address[1] as u64) << 32)
        | ((mac_address[2] as u64) << 24)
        | ((mac_address[3] as u64) << 16)
        | ((mac_address[4] as u64) << 8)
        | (mac_address[5] as u64);

    let network_config = embassy_net::Config::dhcpv4(embassy_net::DhcpConfig::default());
    let stack_resources_ref = STACK_RESOURCES.init(StackResources::<4>::new());

    let (stack_instance, net_runner_instance) = embassy_net::new(
        &mut interfaces_ref.sta, 
        network_config,
        stack_resources_ref,
        mac_u64
    );
    
    let stack = STACK_INIT.init(stack_instance);

    spawner.spawn(net_runner(net_runner_instance)).unwrap();

    info!("WiFi initialized, delaying before starting task...");
    Timer::after(Duration::from_millis(1000)).await;

    let _wifi_task = match spawner.spawn(wifi_connection(wifi_controller_ref, stack, project_config)) {
        Ok(task) => {
            info!("WiFi task spawned successfully");
            task
        },
        Err(e) => {
            info!("Failed to spawn WiFi task: {:?}", e);
            return;
        }
    };
    
    let http_channel = HTTP_CHANNEL.init(Channel::new());
    let http_receiver = http_channel.receiver();
    let http_sender = http_channel.sender();

    let http_task = spawner.spawn(
        http_camera_task(stack, http_receiver, http_sender, project_config));
    if let Ok(_http_task) = http_task {
        info!("HTTP task spawned successfully");
        http_sender.send(HttpMessage::RequestCapture).await;
    } else {
        info!("Failed to spawn HTTP task");
    }

    let http_server_task = spawner.spawn(
        http_server_task(&*stack, display_sender));
    if let Ok(_http_server_task) = http_server_task {
        info!("HTTP Server task spawned succesfully");
    } else {
        info!("Failed to spawn HTTP Server task");
    }


    Timer::after(Duration::from_secs(10)).await;

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}