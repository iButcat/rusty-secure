use heapless::String;

pub mod lcd;
pub use lcd::LcdDisplay;

#[derive(Clone)]
pub enum DisplayMessage {
    Text(String<64>),
    Clear
}

impl DisplayMessage {
    pub fn new_text(text: String<64>) -> Self {
        DisplayMessage::Text(text)
    }

    pub fn new_clear() -> Self {
        DisplayMessage::Clear
    }
}