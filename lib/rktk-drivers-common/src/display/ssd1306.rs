//! SSD1306 OLED display driver

use display_interface::DisplayError;
use embedded_graphics::{
    geometry::Point,
    mono_font::{ascii::FONT_6X10, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::{Dimensions, DrawTarget},
};
use embedded_hal::i2c::I2c as I2cSync;
use embedded_hal_async::i2c::I2c as I2cAsync;
use rktk::drivers::interface::{display::DisplayDriver, DriverBuilder};
use ssd1306::{
    mode::BufferedGraphicsModeAsync, prelude::*, size::DisplaySizeAsync, I2CDisplayInterface,
    Ssd1306Async,
};

pub struct Ssd1306DisplayBuilder<I2C: I2cAsync + I2cSync, SIZE: DisplaySizeAsync>(
    Ssd1306Async<I2CInterface<I2C>, SIZE, BufferedGraphicsModeAsync<SIZE>>,
);

impl<I2C, SIZE> Ssd1306DisplayBuilder<I2C, SIZE>
where
    I2C: I2cAsync + I2cSync,
    SIZE: DisplaySizeAsync,
{
    pub fn new(i2c: I2C, size: SIZE) -> Self {
        let interface = I2CDisplayInterface::new(i2c);
        Self(
            Ssd1306Async::new(interface, size, DisplayRotation::Rotate0)
                .into_buffered_graphics_mode(),
        )
    }

    // pub fn build_sync(mut self) -> Result<Ssd1306Display<I2C, SIZE>, DisplayError> {
    //     self.0.init()?;
    //     Ok(Ssd1306Display(self.0))
    // }
}

impl<I2C, SIZE> DriverBuilder for Ssd1306DisplayBuilder<I2C, SIZE>
where
    I2C: I2cAsync + I2cSync,
    SIZE: DisplaySizeAsync,
{
    type Output = Ssd1306Display<I2C, SIZE>;
    type Error = DisplayError;

    async fn build(mut self) -> Result<Self::Output, Self::Error> {
        self.0.init().await?;
        Ok(Ssd1306Display(self.0))
    }
}

pub struct Ssd1306Display<I2C: I2cAsync + I2cSync, SIZE: DisplaySizeAsync>(
    Ssd1306Async<I2CInterface<I2C>, SIZE, BufferedGraphicsModeAsync<SIZE>>,
);

impl<I2C, SIZE> DisplayDriver for Ssd1306Display<I2C, SIZE>
where
    I2C: I2cAsync + I2cSync,
    SIZE: DisplaySizeAsync + DisplaySize,
{
    const TEXT_STYLE: MonoTextStyle<'static, BinaryColor> = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .background_color(BinaryColor::Off)
        .build();
    const MAX_TEXT_WIDTH: usize = 20;

    // fn flush(&mut self) -> Result<(), DisplayError> {
    //     self.0.flush()
    // }
    async fn flush(&mut self) -> Result<(), DisplayError> {
        self.0.flush().await
    }

    fn clear_buffer(&mut self) {
        self.0.clear_buffer()
    }

    fn calculate_point(col: i32, row: i32) -> Point {
        Point::new((col - 1) * 6, (row - 1) * 10)
    }
}

impl<I2C, SIZE> Dimensions for Ssd1306Display<I2C, SIZE>
where
    I2C: I2cAsync + I2cSync,
    SIZE: DisplaySize + DisplaySizeAsync,
{
    fn bounding_box(&self) -> embedded_graphics::primitives::Rectangle {
        self.0.bounding_box()
    }
}

impl<I2C, SIZE> DrawTarget for Ssd1306Display<I2C, SIZE>
where
    I2C: I2cAsync + I2cSync,
    SIZE: DisplaySize + DisplaySizeAsync,
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
