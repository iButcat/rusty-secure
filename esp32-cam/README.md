# ESP32-CAM

This component uses the ESP32-CAM with the OV2640 camera module for the Rusty Secure system.

## Setup Instructions

1. After cloning the repository, initialize the ESP32-CAM component submodule:

```bash
git submodule update --init
```

2. Check that the esp32-camera component is properly installed in the `components/esp32-camera` directory.

3. Build and flash the firmware:

```bash
cargo build --release
espflash flash --monitor
```

## Notes

- We are using the Espressif camera driver which is written in C and use Rust bindings to utilize it.
- The camera module requires ESP-IDF rather than embassy-rs/esp-hal due to the complexity of the camera driver.
- For more information, see the reference project: https://github.com/jlocash/esp-camera-rs