use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Receiver;

use crate::light::{Led, LedMessage};

#[embassy_executor::task]
pub async fn led_task(
    mut led: Led,
    receiver: Receiver<'static, CriticalSectionRawMutex, LedMessage, 1>,
) {
    loop {
        match receiver.receive().await {
            LedMessage::On => {
                led.set_high().await;
            }
            LedMessage::Off => {
                led.set_low().await;
            }
            LedMessage::Toggle => {
                led.toggle().await;
            }
        }
    }
}
