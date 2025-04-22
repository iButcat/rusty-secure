use esp_wifi::esp_now::EspNow;
use embassy_time::{Duration, Timer};
use embassy_sync::channel::Receiver;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use heapless::Vec;
use log::{info, error};

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
    ) -> Result<Self, ()> {
        Ok(Self {
            esp_now,
            peer_mac,
            receiver
        })
    }

    pub async fn send_capture_command(&mut self) -> Result<(), ()> {
        info!("Sending capture command to camera");
        match self.esp_now.send(&self.peer_mac, b"capture") {
            Ok(waiter) => {
                match waiter.wait() {
                    Ok(_) => {
                        info!("Capture command sent to camera");
                        Ok(())
                    }
                    Err(_) => {
                        info!("Failed to send capture command");
                        Err(())
                    }
                }
            }
            Err(_) => {
                error!("Failed to send capture command");
                Err(())
            }
        }
    }

    pub async fn receive_data(&mut self) -> Result<Vec<u8, 1024>, ()> {
        if let Some(received) = self.esp_now.receive() {
            let data = received.data();
            if !data.is_empty() {
                let mut buffer: Vec<u8, 1024> = Vec::new();
                buffer.extend_from_slice(data).map_err(|_| ())?;
                return Ok(buffer);
            }
        } else {
            Timer::after(Duration::from_millis(500)).await;
        }
        Err(())
    }

    pub async fn handle_incoming_messages(&mut self) {
        match self.receiver.receive().await {
            EspNowCommMessage::SendCaptureCommand => {
                if let Err(_) = self.send_capture_command().await {
                    info!("Failed to send capture command");
                }
            }
            EspNowCommMessage::ImageReceived(data) => {
                info!("Received image data: {} bytes", data.len());
            }
        }
    }
}