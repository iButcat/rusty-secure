use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Receiver;
use heapless::String;

use crate::display::{DisplayMessage, LcdDisplay};

#[embassy_executor::task]
pub async fn display_task(
    mut lcd: LcdDisplay,
    receiver: Receiver<'static, CriticalSectionRawMutex, DisplayMessage, 2>,
) {
    match lcd.init().await {
        Ok(_) => log::info!("LCD initialized successfully"),
        Err(e) => log::error!("LCD initialization failed: {:?}", e),
    }

    loop {
        match receiver.receive().await {
            DisplayMessage::Text(text) => {
                if let Err(e) = lcd.clear().await {
                    log::error!("Failed to clear LCD: {:?}", e);
                }
                if let Err(e) = lcd.write_text(&text).await {
                    log::error!("Failed to write text to LCD: {:?}", e);
                }
            }
            DisplayMessage::Clear => {
                if let Err(e) = lcd.clear().await {
                    log::error!("Failed to clear LCD: {:?}", e);
                }
            }
            DisplayMessage::AuthStatus(is_authorised) => {
                let mut text = String::<64>::new();
                if is_authorised {
                    if text.push_str("Authorised").is_err() {
                        log::error!("Failed to create 'Authorised' string");
                        continue;
                    }
                    log::info!("Display: Showing Authorised");
                } else {
                    if text.push_str("Not Authorised").is_err() {
                        log::error!("Failed to create 'Not Authorised' string");
                        continue;
                    }
                    log::info!("Display: Showing Not Authorised");
                }

                if let Err(e) = lcd.clear().await {
                    log::error!("Failed to clear LCD for auth status: {:?}", e);
                }
                if let Err(e) = lcd.write_text(&text).await {
                    log::error!("Failed to write auth status to LCD: {:?}", e);
                }
            }
        }
    }
}
