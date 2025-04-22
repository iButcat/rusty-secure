use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, 
    channel::Channel
};
use std::vec::Vec;

mod esp_now_comm;
pub use esp_now_comm::EspNowComm;

#[derive(Debug)]
pub enum EspNowCommMessage {
    ReceiveCaptureCommand(),
    SendImage(Vec<u8>),
}

impl EspNowCommMessage {
    pub fn new_receive_capture_command() -> Self {
        EspNowCommMessage::ReceiveCaptureCommand()
    }

    pub fn new_send_image(image: Vec<u8>) -> Self {
        EspNowCommMessage::SendImage(image)
    }
}

pub static ESP_NOW_CHANNEL: Channel<CriticalSectionRawMutex, EspNowCommMessage, 2> = Channel::new();

use esp_idf_svc::espnow::EspNow;
use esp_idf_svc::sys::EspError;
use log::info;

pub fn setup_esp_now_receiver_callback(esp_now: &EspNow<'static>) -> Result<(), EspError> {
    esp_now.register_recv_cb(|receiver_info, data| {
        info!("Received data from {:?} with length {}", receiver_info.src_addr, data.len());
        let mut buffer: Vec<u8> = Vec::with_capacity(data.len());
        buffer.extend_from_slice(data);

        if data == b"capture" {
            if let Err(err) = ESP_NOW_CHANNEL
                .try_send(EspNowCommMessage::new_receive_capture_command()) {
                info!("Failed to send receive capture command: {:?}", err);
            }
        } else {
            info!("Received unknown command");
        }
    })
}