//! SSD1306 OLED display driver

use display_interface::DisplayError;
use embedded_hal::i2c::I2c as I2cSync;
use embedded_hal_async::i2c::I2c as I2cAsync;
use rktk::drivers::interface::display::DisplayDriver;
pub use ssd1306::prelude;
use ssd1306::{
    I2CDisplayInterface, Ssd1306Async, mode::BufferedGraphicsModeAsync, prelude::*,
    size::DisplaySizeAsync,
};

type Ssd1306<I2C, SIZE> = Ssd1306Async<I2CInterface<I2C>, SIZE, BufferedGraphicsModeAsync<SIZE>>;

pub struct Ssd1306Driver<I2C: I2cAsync + I2cSync, SIZE: DisplaySizeAsync>(Ssd1306<I2C, SIZE>);

impl<I2C, SIZE> Ssd1306Driver<I2C, SIZE>
where
    I2C: I2cAsync + I2cSync + 'static,
    SIZE: DisplaySizeAsync + DisplaySize + 'static,
{
    pub fn new(i2c: I2C, size: SIZE, rotation: DisplayRotation) -> Self {
        let interface = I2CDisplayInterface::new(i2c);
        Self(Ssd1306Async::new(interface, size, rotation).into_buffered_graphics_mode())
    }
}

impl<I2C, SIZE> DisplayDriver for Ssd1306Driver<I2C, SIZE>
where
    I2C: I2cAsync + I2cSync + 'static,
    SIZE: DisplaySizeAsync + DisplaySize + 'static,
{
    async fn init(&mut self) -> Result<(), DisplayError> {
        self.0.init().await
    }

    async fn flush(&mut self) -> Result<(), DisplayError> {
        self.0.flush().await
    }

    async fn set_brightness(&mut self, brightness: u8) -> Result<(), DisplayError> {
        self.0
            .set_brightness(Brightness::custom(1, brightness))
            .await
    }

    async fn set_display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        self.0.set_display_on(on).await
    }

    type Display = Ssd1306Async<I2CInterface<I2C>, SIZE, BufferedGraphicsModeAsync<SIZE>>;
}

impl<I2C, SIZE> AsRef<Ssd1306<I2C, SIZE>> for Ssd1306Driver<I2C, SIZE>
where
    I2C: I2cAsync + I2cSync + 'static,
    SIZE: DisplaySizeAsync + DisplaySize + 'static,
{
    fn as_ref(&self) -> &Ssd1306<I2C, SIZE> {
        &self.0
    }
}

impl<I2C, SIZE> AsMut<Ssd1306<I2C, SIZE>> for Ssd1306Driver<I2C, SIZE>
where
    I2C: I2cAsync + I2cSync + 'static,
    SIZE: DisplaySizeAsync + DisplaySize + 'static,
{
    fn as_mut(&mut self) -> &mut Ssd1306<I2C, SIZE> {
        &mut self.0
    }
}
