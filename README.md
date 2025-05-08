# Rusty Secure 🔒

**Rusty Secure** is a comprehensive IoT security system implemented entirely in Rust, demonstrating advanced embedded development with ESP32 microcontrollers, WiFi connectivity, and a complete client-server architecture.

## System Overview

This project demonstrates a fully-functional IoT security system with the following components:

1. **ESP32 Main Controller** (`esp32-main`): A baremetal `no_std` Rust implementation running on ESP32
2. **ESP32 Camera Module** (`esp32-cam`): ESP32 with camera using `esp-idf` and standard library
3. **API Server** (`api-server`): Rust-based web server handling image processing and authorization
4. **Desktop Client** (`desktop-client`): Cross-platform GUI application for system management (planned)


## TODO: add the system architecture 

![System Architecture](https://raw.githubusercontent.com/username/rusty-secure/main/docs/architecture.png)

## Key Features

- **Dual ESP32 Architecture**: Showcases both `no_std` (ESP32-Main) and std-based (ESP32-Cam) Rust embedded programming approaches
- **Multi-sensor Integration**: Ultrasonic distance sensor, camera module, LED indicators, and LCD display
- **WiFi Connectivity**: Secure communication between devices using WiFi
- **Image Capture & Analysis**: On-demand photo capture with server-side processing
- **Authorization System**: API-based security authorization flow
- **Rust Throughout**: 100% Rust implementation across embedded, server, and client components

## Components

### ESP32 Main Controller (`esp32-main`)
- Implemented with `no_std` Rust using Embassy framework
- Runs on standard ESP32 microcontroller
- Features:
  - Distance sensor integration (ultrasonic)
  - I2C LCD display for status information
  - LED indicators for system status
  - WiFi client for communication with ESP32-CAM
  - Triggers image captures when motion is detected

### ESP32 Camera Module (`esp32-cam`)
- Implemented with std Rust and ESP-IDF bindings
- Runs on ESP32-CAM board with OV2640 camera
- Features:
  - Embedded web server for on-demand image capture
  - Camera control with configurable settings
  - Flash LED control
  - Image transmission to API server
  - Secure WiFi communication

### API Server (`api-server`)
- Rust-based web server using Actix-Web
- Features:
  - Image reception and storage
  - Authorization management
  - Status reporting
  - RESTful API design

### Desktop Client (`desktop-client`)
- Planned Rust GUI application using Iced toolkit
- Features (planned):
  - Google OAuth authentication
  - Image review and management
  - Authorization controls
  - Real-time system monitoring

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [ESP-IDF toolchain](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/)
- [espup](https://github.com/esp-rs/espup) for Rust on ESP32 setup
- ESP32 development board
- ESP32-CAM board with OV2640 camera
- Ultrasonic distance sensor (HC-SR04)
- I2C LCD display (16x2)
- Jumper wires and breadboard

### Environment Setup

1. Install Rust and required components:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup update
   ```

2. Install ESP32 toolchain using espup:
   ```bash
   cargo install espup
   espup install
   ```

3. Add necessary targets:
   ```bash
   rustup target add riscv32imc-unknown-none-elf
   rustup target add xtensa-esp32-none-elf
   rustup target add xtensa-esp32s3-none-elf
   ```

### Configuration

Each component requires proper configuration before use:

1. Create `secrets.rs` files in each component:
   - `esp32-main/src/config/secrets.rs`
   - `esp32-cam/src/config/secrets.rs`
   - `api-server/src/config/secrets.rs`
   - `desktop-client/src/config/secrets.rs`

2. Configure WiFi credentials:
   ```rust
   // Example for esp32-cam/src/config/secrets.rs and esp32-main/src/config/secrets.rs
   pub const WIFI_SSID: &str = "your_wifi_ssid";
   pub const WIFI_PASSWORD: &str = "your_wifi_password";
   pub const API_URL: &str = "http://your_api_server_ip:8080/analyse";
   ```

### Hardware Setup

#### ESP32 Main Controller
Connect components to your ESP32 board:
- Ultrasonic Sensor:
  - Trigger: GPIO5
  - Echo: GPIO19
- I2C LCD Display:
  - SDA: GPIO21
  - SCL: GPIO22
- LED Indicator: GPIO2

#### ESP32-CAM
The ESP32-CAM board already includes the OV2640 camera module and a flash LED on GPIO4.

### Building & Flashing

#### ESP32 Main Controller
```bash
cd esp32-main
cargo +esp run --release
```

#### ESP32-CAM Module
```bash
cd esp32-cam
cargo +esp run --release
```

#### API Server
```bash
cd api-server
cargo run --release
```

## Usage

1. Power on the ESP32 Main Controller
2. Power on the ESP32-CAM module
3. Start the API server
4. The system will:
   - Display status information on the LCD
   - Monitor for motion using the ultrasonic sensor
   - Trigger the ESP32-CAM to capture images when motion is detected
   - Process images through the API server for authorization
   - Indicate authorization status via the LED

## Project Status

- **ESP32 Main Controller**: Functional prototype
- **ESP32-CAM Module**: Functional prototype
- **API Server**: Basic implementation, needs enhancement
- **Desktop Client**: Planned, not yet implemented

## Contributing

Contributions are welcome! Here are some areas where help is needed:

1. Enhancing the API server functionality
2. Implementing the desktop client
3. Improving documentation and examples
4. Adding new features to the ESP32 components

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgements

- The [Embassy](https://github.com/embassy-rs/embassy) project for embedded async Rust
- [esp-rs](https://github.com/esp-rs) for Rust support on ESP32
- [Actix Web](https://github.com/actix/actix-web) for the API server
- [Iced](https://github.com/iced-rs/iced) for the desktop client UI
