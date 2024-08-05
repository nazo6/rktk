use display_interface::DisplayError;
use embedded_graphics::{
    geometry::Point,
    mono_font::{ascii::FONT_6X10, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::{Dimensions, DrawTarget},
};
use embedded_hal::i2c::I2c as I2cSync;
use embedded_hal_async::i2c::I2c as I2cAsync;
use rktk::interface::{display::DisplayDriver, DriverBuilder};
use ssd1306::{
    mode::{BufferedGraphicsMode, DisplayConfig as _},
    prelude::I2CInterface,
    rotation::DisplayRotation,
    size::DisplaySize,
    I2CDisplayInterface, Ssd1306,
};

pub struct Ssd1306DisplayBuilder<I2C: I2cAsync + I2cSync, SIZE: DisplaySize>(
    Ssd1306<I2CInterface<I2C>, SIZE, BufferedGraphicsMode<SIZE>>,
);

impl<I2C, SIZE> Ssd1306DisplayBuilder<I2C, SIZE>
where
    I2C: I2cAsync + I2cSync,
    SIZE: DisplaySize,
{
    pub fn new(i2c: I2C, size: SIZE) -> Self {
        let interface = I2CDisplayInterface::new(i2c);
        Self(Ssd1306::new(interface, size, DisplayRotation::Rotate0).into_buffered_graphics_mode())
    }

    pub fn build_sync(mut self) -> Result<Ssd1306Display<I2C, SIZE>, DisplayError> {
        self.0.init()?;
        Ok(Ssd1306Display(self.0))
    }
}

impl<I2C, SIZE> DriverBuilder for Ssd1306DisplayBuilder<I2C, SIZE>
where
    I2C: I2cAsync + I2cSync,
    SIZE: DisplaySize,
{
    type Output = Ssd1306Display<I2C, SIZE>;
    type Error = DisplayError;

    async fn build(mut self) -> Result<Self::Output, Self::Error> {
        self.0.init()?;
        Ok(Ssd1306Display(self.0))
    }
}

pub struct Ssd1306Display<I2C: I2cAsync + I2cSync, SIZE: DisplaySize>(
    Ssd1306<I2CInterface<I2C>, SIZE, BufferedGraphicsMode<SIZE>>,
);

impl<I2C, SIZE> DisplayDriver for Ssd1306Display<I2C, SIZE>
where
    I2C: I2cAsync + I2cSync,
    SIZE: DisplaySize,
{
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

    const TEXT_STYLE: MonoTextStyle<'static, BinaryColor> = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .background_color(BinaryColor::Off)
        .build();
}

impl<I2C, SIZE> Dimensions for Ssd1306Display<I2C, SIZE>
where
    I2C: I2cAsync + I2cSync,
    SIZE: DisplaySize,
{
    fn bounding_box(&self) -> embedded_graphics::primitives::Rectangle {
        self.0.bounding_box()
    }
}

impl<I2C, SIZE> DrawTarget for Ssd1306Display<I2C, SIZE>
where
    I2C: I2cAsync + I2cSync,
    SIZE: DisplaySize,
{
    type Color = BinaryColor;

    type Error = DisplayError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        self.0.draw_iter(pixels)
    }
}
