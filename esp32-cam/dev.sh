#!/usr/bin/env bash
# ESP32-CAM development build, flash & monitor
# Usage: ./dev.sh [--erase] [--port /dev/tty.usbserial-140]
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
TARGET_DIR="$SCRIPT_DIR/../target/xtensa-esp32-espidf/debug"
PROFILE="debug"
ERASE=false
PORT=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --erase)
            ERASE=true
            shift
            ;;
        --port)
            PORT="$2"
            shift 2
            ;;
        *)
            echo "Usage: $0 [--erase] [--port <serial-port>]"
            exit 1
            ;;
    esac
done

PORT_FLAG=""
if [[ -n "$PORT" ]]; then
    PORT_FLAG="--port $PORT"
fi

if $ERASE; then
    echo "==> Erasing flash..."
    espflash erase-flash $PORT_FLAG
fi

echo "==> Building (dev)..."
cargo +esp build

# Find the bootloader and partition table from the esp-idf-sys build output
BUILD_DIR=$(find "$SCRIPT_DIR/../target/xtensa-esp32-espidf/$PROFILE/build" \
    -maxdepth 1 -name "esp-idf-sys-*" -type d | sort -t- -k5 | tail -1)
BOOTLOADER="$BUILD_DIR/out/build/bootloader/bootloader.bin"
PARTITION_TABLE="$BUILD_DIR/out/build/partition_table/partition-table.bin"

if [[ ! -f "$BOOTLOADER" ]]; then
    echo "ERROR: Bootloader not found at $BOOTLOADER"
    echo "Try: cargo clean && cargo +esp build"
    exit 1
fi

echo "==> Using bootloader: $BOOTLOADER"
echo "==> Flashing & monitoring..."
espflash flash \
    --bootloader "$BOOTLOADER" \
    --partition-table "$PARTITION_TABLE" \
    "$TARGET_DIR/esp32-cam" \
    --monitor $PORT_FLAG
