use embassy_sync::channel::Receiver;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

use crate::display::{LcdDisplay, DisplayMessage};

#[embassy_executor::task]
pub async fn display_task(
    mut lcd: LcdDisplay,
    receiver: Receiver<'static, CriticalSectionRawMutex, DisplayMessage, 2>
) {    
    match lcd.init().await {
        Ok(_) => log::info!("LCD initialized successfully"),
        Err(_) => log::warn!("LCD initialization failed!")
    }

    loop {
        match receiver.receive().await {
            DisplayMessage::Text(text) => {
                let _ = lcd.clear().await;
                let _ = lcd.write_text(&text).await;
            }
            DisplayMessage::Clear => {
                let _ = lcd.clear().await;
            }
        }
    }
}