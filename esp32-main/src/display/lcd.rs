use embassy_time::{Duration, Timer};
use esp_hal::i2c::master::I2c;
use esp_hal::Blocking;
use log::info;

const LCD_ADDRESS: u8 = 0x27;
const LCD_ENABLE: u8 = 0b0000_0100;
const LCD_BACKLIGHT: u8 = 0b0000_1000;
const LCD_DATA: u8 = 0b0000_0001;

pub struct LcdDisplay {
    i2c: I2c<'static, Blocking>,
}

impl LcdDisplay {
    pub fn new(i2c: I2c<'static, Blocking>) -> Self {
        Self { i2c }
    }

    pub async fn init(&mut self) -> Result<(), ()> {
        info!("Initializing LCD...");
        Timer::after(Duration::from_millis(100)).await;

        self.write_4bits(0x30).await?;
        Timer::after(Duration::from_millis(10)).await;
        
        self.write_4bits(0x30).await?;
        Timer::after(Duration::from_micros(150)).await;
        
        self.write_4bits(0x30).await?;
        self.write_4bits(0x20).await?;

        self.write_command(0x28).await?;
        self.write_command(0x0C).await?;
        self.write_command(0x06).await?;
        self.write_command(0x01).await?;
        Timer::after(Duration::from_millis(2)).await;

        Ok(())
    }

    async fn write_4bits(&mut self, value: u8) -> Result<(), ()> {
        let data = value | LCD_BACKLIGHT;
        info!("I2C write: addr=0x{:02X}, data=0x{:02X}", LCD_ADDRESS, data);
        
        match self.i2c.write(LCD_ADDRESS, &[data]) {
            Ok(_) => {},
            Err(e) => {
                info!("I2C write failed: {:?}", e);
                return Err(());
            }
        }
        self.i2c.write(LCD_ADDRESS, &[data | LCD_ENABLE]).map_err(|_| ())?;
        self.i2c.write(LCD_ADDRESS, &[data & !LCD_ENABLE]).map_err(|_| ())?;
        Timer::after(Duration::from_micros(1)).await;
        Ok(())
    }

    pub async fn write_text(&mut self, text: &str) -> Result<(), ()> {
        for byte in text.bytes() {
            self.write_data(byte).await?;
        }
        Ok(())
    }

    async fn write_command(&mut self, cmd: u8) -> Result<(), ()> {
        self.write_4bits(cmd & 0xF0).await?;
        self.write_4bits((cmd << 4) & 0xF0).await?;
        Ok(())
    }

    async fn write_data(&mut self, data: u8) -> Result<(), ()> {
        self.write_4bits((data & 0xF0) | LCD_DATA).await?;
        self.write_4bits((data << 4) | LCD_DATA).await?;
        Ok(())
    }

    pub async fn clear(&mut self) -> Result<(), ()> {
        self.write_command(0x01).await?;
        Timer::after(Duration::from_millis(2)).await;
        Ok(())
    }
}
