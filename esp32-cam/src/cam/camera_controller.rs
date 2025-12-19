use esp_idf_sys::EspError;
use log::info;
use std::vec::Vec;

use crate::esp_cam::Camera;

pub struct CameraController<'a> {
    camera: Camera<'a>,
}

impl<'a> CameraController<'a> {
    /// Create a new CameraController.
    /// This is also on top of binding the camera to the esp32-cam.
    ///
    /// The parameters are passed in the following order:
    /// 1. `pin_pwdn`: Power-down pin (dual-mode)
    /// 2. `pin_xclk`: External clock pin (dual-mode)
    /// 3. `pin_d0`: Data pin 0 (dual-mode)
    /// 4. `pin_d1`: Data pin 1 (dual-mode)
    /// 5. `pin_d2`: Data pin 2 (dual-mode)
    /// 6. `pin_d3`: Data pin 3 (dual-mode)
    /// 7. `pin_d4`: Data pin 4 (input-only)
    /// 8. `pin_d5`: Data pin 5 (input-only)
    /// 9. `pin_d6`: Data pin 6 (input-only)
    /// 10. `pin_d7`: Data pin 7 (input-only)
    /// 11. `pin_vsync`: Vertical sync pin (dual-mode)
    /// 12. `pin_href`: Horizontal reference pin (dual-mode)
    /// 13. `pin_pclk`: Pixel clock pin (dual-mode)
    /// 14. `pin_sda`: SCCB SDA pin (dual-mode)
    /// 15. `pin_scl`: SCCB SCL pin (dual-mode)
    /// 16. `pixel_format`: e.g. `camera::pixformat_t::PIXFORMAT_JPEG`
    /// 17. `frame_size`: e.g. `camera::framesize_t::FRAMESIZE_UXGA`
    pub fn new(
        pin_pwdn: impl esp_idf_hal::peripheral::Peripheral<
                P = impl esp_idf_hal::gpio::InputPin + esp_idf_hal::gpio::OutputPin,
            > + 'a,
        pin_xclk: impl esp_idf_hal::peripheral::Peripheral<
                P = impl esp_idf_hal::gpio::InputPin + esp_idf_hal::gpio::OutputPin,
            > + 'a,
        pin_d0: impl esp_idf_hal::peripheral::Peripheral<
                P = impl esp_idf_hal::gpio::InputPin + esp_idf_hal::gpio::OutputPin,
            > + 'a,
        pin_d1: impl esp_idf_hal::peripheral::Peripheral<
                P = impl esp_idf_hal::gpio::InputPin + esp_idf_hal::gpio::OutputPin,
            > + 'a,
        pin_d2: impl esp_idf_hal::peripheral::Peripheral<
                P = impl esp_idf_hal::gpio::InputPin + esp_idf_hal::gpio::OutputPin,
            > + 'a,
        pin_d3: impl esp_idf_hal::peripheral::Peripheral<
                P = impl esp_idf_hal::gpio::InputPin + esp_idf_hal::gpio::OutputPin,
            > + 'a,
        pin_d4: impl esp_idf_hal::peripheral::Peripheral<P = impl esp_idf_hal::gpio::InputPin> + 'a,
        pin_d5: impl esp_idf_hal::peripheral::Peripheral<P = impl esp_idf_hal::gpio::InputPin> + 'a,
        pin_d6: impl esp_idf_hal::peripheral::Peripheral<P = impl esp_idf_hal::gpio::InputPin> + 'a,
        pin_d7: impl esp_idf_hal::peripheral::Peripheral<P = impl esp_idf_hal::gpio::InputPin> + 'a,
        pin_vsync: impl esp_idf_hal::peripheral::Peripheral<
                P = impl esp_idf_hal::gpio::InputPin + esp_idf_hal::gpio::OutputPin,
            > + 'a,
        pin_href: impl esp_idf_hal::peripheral::Peripheral<
                P = impl esp_idf_hal::gpio::InputPin + esp_idf_hal::gpio::OutputPin,
            > + 'a,
        pin_pclk: impl esp_idf_hal::peripheral::Peripheral<
                P = impl esp_idf_hal::gpio::InputPin + esp_idf_hal::gpio::OutputPin,
            > + 'a,
        pin_sda: impl esp_idf_hal::peripheral::Peripheral<
                P = impl esp_idf_hal::gpio::InputPin + esp_idf_hal::gpio::OutputPin,
            > + 'a,
        pin_scl: impl esp_idf_hal::peripheral::Peripheral<
                P = impl esp_idf_hal::gpio::InputPin + esp_idf_hal::gpio::OutputPin,
            > + 'a,
        pixel_format: esp_idf_sys::camera::pixformat_t,
        frame_size: esp_idf_sys::camera::framesize_t,
    ) -> Result<Self, EspError> {
        let camera = crate::esp_cam::Camera::new(
            pin_pwdn,
            pin_xclk,
            pin_d0,
            pin_d1,
            pin_d2,
            pin_d3,
            pin_d4,
            pin_d5,
            pin_d6,
            pin_d7,
            pin_vsync,
            pin_href,
            pin_pclk,
            pin_sda,
            pin_scl,
            pixel_format,
            frame_size,
        )?;
        Ok(Self { camera })
    }

    /// Access the framebuffer captured by the camera.
    pub fn get_framebuffer(&self) -> Option<crate::esp_cam::FrameBuffer> {
        self.camera.get_framebuffer()
    }

    /// Access the camera sensor (for further configuration).
    pub fn sensor(&self) -> crate::esp_cam::CameraSensor {
        self.camera.sensor()
    }

    pub fn capture(&self) -> Option<Vec<u8>> {
        if let Some(frame_buffer) = self.camera.get_framebuffer() {
            let data = frame_buffer.data();
            let mut buffer = Vec::with_capacity(data.len());
            buffer.extend_from_slice(data);
            Some(buffer)
        } else {
            info!("Failed to get framebuffer");
            None
        }
    }
}
