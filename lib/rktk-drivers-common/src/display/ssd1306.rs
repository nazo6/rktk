//! SSD1306 OLED display driver

use display_interface::DisplayError;
use embedded_graphics::{
    geometry::Point,
    mono_font::{MonoTextStyle, MonoTextStyleBuilder, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    prelude::{Dimensions, DrawTarget},
};
use embedded_hal::i2c::I2c as I2cSync;
use embedded_hal_async::i2c::I2c as I2cAsync;
use rktk::drivers::interface::display::DisplayDriver;
pub use ssd1306::prelude;
use ssd1306::{
    I2CDisplayInterface, Ssd1306Async, mode::BufferedGraphicsModeAsync, prelude::*,
    size::DisplaySizeAsync,
};

pub struct Ssd1306Display<I2C: I2cAsync + I2cSync, SIZE: DisplaySizeAsync>(
    Ssd1306Async<I2CInterface<I2C>, SIZE, BufferedGraphicsModeAsync<SIZE>>,
);

impl<I2C, SIZE> Ssd1306Display<I2C, SIZE>
where
    I2C: I2cAsync + I2cSync + 'static,
    SIZE: DisplaySizeAsync + DisplaySize + 'static,
{
    pub fn new(i2c: I2C, size: SIZE) -> Self {
        let interface = I2CDisplayInterface::new(i2c);
        Self(
            Ssd1306Async::new(interface, size, DisplayRotation::Rotate0)
                .into_buffered_graphics_mode(),
        )
    }
}

impl<I2C, SIZE> DisplayDriver for Ssd1306Display<I2C, SIZE>
where
    I2C: I2cAsync + I2cSync + 'static,
    SIZE: DisplaySizeAsync + DisplaySize + 'static,
{
    const TEXT_STYLE: MonoTextStyle<'static, BinaryColor> = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .background_color(BinaryColor::Off)
        .build();
    const MAX_TEXT_WIDTH: usize = 20;

    async fn init(&mut self) -> Result<(), DisplayError> {
        self.0.init().await
    }

    async fn flush(&mut self) -> Result<(), DisplayError> {
        self.0.flush().await
    }

    fn clear_buffer(&mut self) {
        self.0.clear_buffer()
    }

    fn calculate_point(col: i32, row: i32) -> Point {
        Point::new((col - 1) * 6, (row - 1) * 10)
    }

    async fn set_brightness(&mut self, brightness: u8) -> Result<(), DisplayError> {
        self.0
            .set_brightness(Brightness::custom(1, brightness))
            .await
    }

    async fn set_display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        self.0.set_display_on(on).await
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
