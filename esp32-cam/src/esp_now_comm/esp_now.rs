use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Timer};
use log::info;
use std::sync::Arc;

use crate::cam::camera_controller::CameraController;
use crate::esp_now_comm::EspNowComm;

#[embassy_executor::task]
pub async fn handle_esp_now_task(
    mut comm: EspNowComm<'static>,
    camera: Arc<Mutex<NoopRawMutex, CameraController<'static>>>,
) {
    loop {
        if comm.receive_capture_command().await.is_ok() {
            info!("Capture command received");
            let captured = {
                let cam = camera.lock().await;
                cam.capture()
            };
            if let Some(image) = captured {
                if let Err(e) = comm.send_image_chunked(image).await {
                    info!("Failed to send image: {}", e);
                }
            } else {
                info!("Failed to capture image");
            }
        }
        Timer::after(Duration::from_millis(100)).await;
    }
}
