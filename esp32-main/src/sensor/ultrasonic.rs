use esp_hal::gpio::{Output, Input};
use embassy_time::{Duration, Timer};
use log::info;

use crate::error::Error;

pub struct UltrasonicSensor {
    trigger: Output<'static>,
    echo: Input<'static>
}

impl UltrasonicSensor {
    pub fn new(trigger: Output<'static>, echo: Input<'static>) -> Self {
        Self { trigger, echo }
    }

    pub async fn measure_distance(&mut self) -> Result<u32, Error> {
        self.trigger.set_low();
        Timer::after(Duration::from_micros(2)).await;
        self.trigger.set_high();
        Timer::after(Duration::from_micros(10)).await;
        self.trigger.set_low();

        let start = embassy_time::Instant::now();
        while self.echo.is_low() {
            if start.elapsed() > Duration::from_millis(100) {
                return Err(Error::Timeout);
            }
        }

        let pulse_start = embassy_time::Instant::now();
        while self.echo.is_high() {
            if pulse_start.elapsed() > Duration::from_millis(100) {
                return Err(Error::Timeout);
            }
        }

        let pulse_duration = pulse_start.elapsed();

        let distance_cm = (pulse_duration.as_micros() as u32 * 343) / 20000;
        
        Ok(distance_cm)
    }
}