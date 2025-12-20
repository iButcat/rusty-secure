use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::{Receiver, Sender};
use embassy_time::{Duration, Timer};
use heapless::String;
use log::{error, info};

use crate::display::DisplayMessage;
use crate::sensor::{SensorMessage, UltrasonicSensor};

const DISTANCE_THRESHOLD_CM: u32 = 20;

#[embassy_executor::task]
pub async fn sensor_task(
    mut sensor: UltrasonicSensor,
    receiver: Receiver<'static, CriticalSectionRawMutex, SensorMessage, 1>,
    display_sender: Sender<'static, CriticalSectionRawMutex, DisplayMessage, 2>,
) {
    let mut last_status = true;
    let mut measuring = false;

    let mut init_text: String<64> = String::new();
    let _ = init_text.push_str("Initializing...");
    display_sender.send(DisplayMessage::Text(init_text)).await;

    measuring = true;

    loop {
        if let Ok(message) = receiver.try_receive() {
            match message {
                SensorMessage::StartMeasurement => {
                    measuring = true;
                    info!("Starting measurement");
                }
                SensorMessage::StopMeasurement => {
                    measuring = false;
                    info!("Stopping measurement");
                }
            }
        }

        if measuring {
            match sensor.measure_distance().await {
                Ok(distance) => {
                    let current_status = distance >= DISTANCE_THRESHOLD_CM;

                    if current_status != last_status {
                        let mut text: String<64> = String::new();
                        if !current_status {
                            let _ = text.push_str("Person detected with distance");
                        } else {
                            let _ = text.push_str("Person not detected");
                        }
                        display_sender.send(DisplayMessage::Text(text)).await;
                        last_status = current_status;
                    }
                }
                Err(e) => {
                    error!("Error measuring distance: {}", e);
                    return;
                }
            }
            Timer::after(Duration::from_millis(100)).await;
        }
    }
}
