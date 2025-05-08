use heapless::String;

pub mod lcd;
pub use lcd::LcdDisplay;

#[derive(Clone)]
pub enum DisplayMessage {
    Text(String<64>),
    Clear,
    AuthStatus(bool)
}

impl DisplayMessage {
    pub fn new_text(text: String<64>) -> Self {
        DisplayMessage::Text(text)
    }

    pub fn new_clear() -> Self {
        DisplayMessage::Clear
    }

    pub fn new_auth_status(status: bool) -> Self {
        DisplayMessage::AuthStatus(status)
    }
}