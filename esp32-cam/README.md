# ESP32-CAM

This component uses the ESP32-CAM with the OV2640 camera module for the Rusty Secure system.

## Setup Instructions

1. After cloning the repository, initialize the ESP32-CAM component submodule:

```bash
git submodule update --init
```

2. Check that the esp32-camera component is properly installed in the `components/esp32-camera` directory.

3. Install the ESP Rust toolchain (`channel = "esp"` in `rust-toolchain.toml`). See [esp-rs/rust-build](https://github.com/esp-rs/rust-build) for instructions.

## Build & Flash

Two shell scripts are provided for convenience:

### Development (debug build)

```bash
./dev.sh                           # build, flash & monitor
./dev.sh --erase                   # full erase before flashing
./dev.sh --port /dev/tty.usbserial-140  # specify serial port
```

### Release (optimized build)

```bash
./release.sh                       # build, flash & monitor
./release.sh --erase               # full erase before flashing
./release.sh --port /dev/tty.usbserial-140  # specify serial port
```

Use `--erase` when switching between bootloader versions or after major sdkconfig changes.

### Manual commands

```bash
cargo +esp build                   # dev build
cargo +esp build --release         # release build
espflash flash target/xtensa-esp32-espidf/release/esp32-cam --monitor
```

## Architecture

- Uses ESP-IDF (std) with FreeRTOS — the camera driver requires the full ESP-IDF framework.
- HTTP server listens for `/capture` requests, triggers the camera, and uploads the image to the API server.
- The Espressif camera driver is written in C; Rust bindings are generated at build time from `components/bindings.h`.

## Notes

- PSRAM (SPIRAM) is enabled in `sdkconfig.defaults` for camera frame buffers.
- The flash LED on GPIO4 is toggled during capture for lighting.
- For more information, see the reference project: https://github.com/jlocash/esp-camera-rs
