use crate::esp_now_comm::{EspNowComm, EspNowCommMessage};
use embassy_sync::channel::Receiver;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_time::{Duration, Timer};
use log::{info, error};

#[embassy_executor::task]
pub async fn handle_esp_now_task(
    mut comm: EspNowComm<'static>, 
    receiver: Receiver<'static, CriticalSectionRawMutex, EspNowCommMessage, 2>
) {
    loop {
        if let Ok(message) = receiver.try_receive() {
            match message {
                EspNowCommMessage::SendCaptureCommand => {
                    info!("Sending capture command from task");
                    if let Err(_) = comm.send_capture_command().await {
                        error!("Failed to send capture command");
                    }
                }
                // Other type of message, see later if I need it. 
                _ => {}
            }
        }

        match comm.receive_data().await {
            Ok(data) => {
                info!("Received data from ESP-NOW: {} bytes", data.len());
                if !data.is_empty() {
                    info!("Received data: {:?}", data);
                }
            }
            Err(_) => {
                Timer::after(Duration::from_millis(500)).await;
            }
        }
    }
}

#[embassy_executor::task]
pub async fn capture_command_task(mut comm: EspNowComm<'static>) {
    loop {
        Timer::after(Duration::from_secs(30)).await;
        info!("Sending capture command from task");
        // TODO: handle errors and use the result
        if let Err(_) = comm.send_capture_command().await {
            error!("Failed to send capture command");
        }
    }
}