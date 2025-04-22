mod led;
pub use led::*;

#[derive(Clone)]
pub enum LedMessage {
    On, 
    Off, 
    Toggle
}

impl LedMessage {
    pub fn new_on() -> Self {
        LedMessage::On
    }

    pub fn new_off() -> Self {
        LedMessage::Off
    }

    pub fn new_toggle() -> Self {
        LedMessage::Toggle
    }
}