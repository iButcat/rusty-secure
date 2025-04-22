pub mod ultrasonic;
pub use ultrasonic::UltrasonicSensor;

#[derive(Clone)]
pub enum SensorMessage {
    StartMeasurement, 
    StopMeasurement, 
}

impl SensorMessage {
    pub fn new_start() -> Self {
        SensorMessage::StartMeasurement
    }

    pub fn new_stop() -> Self {
        SensorMessage::StopMeasurement
    }
}