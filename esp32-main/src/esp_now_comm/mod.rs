use heapless::Vec;

mod esp_now_comm;
pub use esp_now_comm::EspNowComm;

#[derive(Clone)]
pub enum EspNowCommMessage {
    SendCaptureCommand,
    ImageReceived(Vec<u8, 1024>),
}

impl EspNowCommMessage {
    pub fn new_send_capture_command() -> Self {
        EspNowCommMessage::SendCaptureCommand
    }

    pub fn new_image_received(data: Vec<u8, 1024>) -> Self {
        EspNowCommMessage::ImageReceived(data)
    }
}
