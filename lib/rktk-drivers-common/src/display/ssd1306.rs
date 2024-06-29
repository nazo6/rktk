use display_interface::DisplayError;
use embedded_graphics::geometry::Point;
use embedded_hal::i2c::I2c as I2cSync;
use embedded_hal_async::i2c::I2c as I2cAsync;
use rktk::interface::display::DisplayDriver;
use ssd1306::{
    mode::{BufferedGraphicsMode, DisplayConfig as _},
    prelude::I2CInterface,
    rotation::DisplayRotation,
    size::DisplaySize,
    I2CDisplayInterface, Ssd1306,
};

pub struct Ssd1306Display<I2C: I2cAsync + I2cSync, SIZE: DisplaySize>(
    Ssd1306<I2CInterface<I2C>, SIZE, BufferedGraphicsMode<SIZE>>,
);

impl<I2C: I2cAsync + I2cSync, SIZE: DisplaySize> Ssd1306Display<I2C, SIZE> {
    pub fn new(i2c: I2C, size: SIZE) -> Self {
        let interface = I2CDisplayInterface::new(i2c);

        Self(Ssd1306::new(interface, size, DisplayRotation::Rotate0).into_buffered_graphics_mode())
    }
}

impl<I2C: I2cAsync + I2cSync, SIZE: DisplaySize> DisplayDriver for Ssd1306Display<I2C, SIZE> {
    type DerefTarget = Ssd1306<I2CInterface<I2C>, SIZE, BufferedGraphicsMode<SIZE>>;

    async fn init(&mut self) -> Result<(), DisplayError> {
        self.0.init()
    }

    fn flush(&mut self) -> Result<(), DisplayError> {
        self.0.flush()
    }
    async fn flush_async(&mut self) -> Result<(), DisplayError> {
        self.0.flush_async().await
    }

    fn clear_buffer(&mut self) {
        self.0.clear_buffer()
    }

    fn calculate_point(col: i32, row: i32) -> Point {
        Point::new((col - 1) * 6, (row - 1) * 10)
    }
}

impl<I2C: I2cAsync + I2cSync, SIZE: DisplaySize> core::ops::Deref for Ssd1306Display<I2C, SIZE> {
    fn deref(&self) -> &Self::Target {
        &self.0
    }

    type Target = Ssd1306<I2CInterface<I2C>, SIZE, BufferedGraphicsMode<SIZE>>;
}

impl<I2C: I2cAsync + I2cSync, SIZE: DisplaySize> core::ops::DerefMut for Ssd1306Display<I2C, SIZE> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
