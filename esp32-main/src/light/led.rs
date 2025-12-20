use esp_hal::gpio::Output;

pub struct Led {
    pin: Output<'static>,
}

impl Led {
    pub fn new(pin: Output<'static>) -> Self {
        Self { pin }
    }

    pub async fn set_high(&mut self) {
        self.pin.set_high();
    }

    pub async fn set_low(&mut self) {
        self.pin.set_low();
    }

    pub async fn toggle(&mut self) {
        self.pin.toggle();
    }
}
