use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Receiver;
use embassy_time::{Duration, Timer};
use esp_idf_svc::espnow::EspNow;
use esp_idf_sys::EspError;
use log::info;
use std::vec::Vec;

use crate::esp_now_comm::EspNowCommMessage;

pub struct EspNowComm<'a> {
    esp_now: &'a mut EspNow<'static>,
    peer_mac: [u8; 6],
    receiver: Receiver<'static, CriticalSectionRawMutex, EspNowCommMessage, 2>,
}

impl<'a> EspNowComm<'a> {
    pub fn new(
        esp_now: &'a mut EspNow<'static>,
        peer_mac: [u8; 6],
        receiver: Receiver<'static, CriticalSectionRawMutex, EspNowCommMessage, 2>,
    ) -> Self {
        Self {
            esp_now,
            peer_mac,
            receiver,
        }
    }

    pub async fn receive_capture_command(&mut self) -> Result<(), ()> {
        match self.receiver.receive().await {
            EspNowCommMessage::ReceiveCaptureCommand() => {
                info!("Received capture command");
                Ok(())
            }
            _ => {
                info!("Received unknown command");
                Err(())
            }
        }
    }

    pub async fn send_image_chunked(&mut self, image: Vec<u8>) -> Result<(), EspError> {
        info!("Sending image");
        info!("Image length: {}", image.len());

        let chunk_size = 200;
        let total_chunks = image.len() / chunk_size;
        info!("Total chunks to send: {}", total_chunks);
        for (i, chunk) in image.chunks(chunk_size).enumerate() {
            let mut packet = Vec::with_capacity(5 + chunk.len());
            packet.push(b'C');
            packet.extend_from_slice(&(total_chunks as u16).to_be_bytes());
            packet.extend_from_slice(&(i as u16).to_le_bytes());
            packet.extend_from_slice(chunk);

            info!("Sending chunk {} of {}", i, total_chunks);
            self.esp_now.send(self.peer_mac, &packet)?;

            Timer::after(Duration::from_millis(10)).await;
        }
        info!("Image sent");
        Ok(())
    }
}
