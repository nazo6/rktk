use display_interface::DisplayError;
use rktk::interface::display::Display;
use ssd1306::{
    mode::{BufferedGraphicsMode, DisplayConfig as _},
    prelude::I2CInterface,
    rotation::DisplayRotation,
    size::DisplaySize,
    I2CDisplayInterface, Ssd1306,
};

pub struct Ssd1306Display<
    I2C: embedded_hal_async::i2c::I2c + embedded_hal::i2c::I2c,
    SIZE: DisplaySize,
>(Ssd1306<I2CInterface<I2C>, SIZE, BufferedGraphicsMode<SIZE>>);

impl<I2C: embedded_hal_async::i2c::I2c + embedded_hal::i2c::I2c, SIZE: DisplaySize>
    Ssd1306Display<I2C, SIZE>
{
    pub fn new(i2c: I2C, size: SIZE) -> Result<Self, DisplayError> {
        let interface = I2CDisplayInterface::new(i2c);

        let mut display =
            Ssd1306::new(interface, size, DisplayRotation::Rotate0).into_buffered_graphics_mode();
        display.init()?;

        Ok(Self(display))
    }
}

impl<I2C: embedded_hal_async::i2c::I2c + embedded_hal::i2c::I2c, SIZE: DisplaySize> Display
    for Ssd1306Display<I2C, SIZE>
{
    type DerefTarget = Ssd1306<I2CInterface<I2C>, SIZE, BufferedGraphicsMode<SIZE>>;

    fn flush(&mut self) -> Result<(), DisplayError> {
        self.0.flush()
    }
    async fn flush_async(&mut self) -> Result<(), DisplayError> {
        self.0.flush_async().await
    }

    fn clear_buffer(&mut self) {
        self.0.clear_buffer()
    }
}

impl<I2C: embedded_hal_async::i2c::I2c + embedded_hal::i2c::I2c, SIZE: DisplaySize> core::ops::Deref
    for Ssd1306Display<I2C, SIZE>
{
    fn deref(&self) -> &Self::Target {
        &self.0
    }

    type Target = Ssd1306<I2CInterface<I2C>, SIZE, BufferedGraphicsMode<SIZE>>;
}

impl<I2C: embedded_hal_async::i2c::I2c + embedded_hal::i2c::I2c, SIZE: DisplaySize>
    core::ops::DerefMut for Ssd1306Display<I2C, SIZE>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
