use core::ops::DerefMut;

use display_interface::DisplayError;
use embedded_graphics::{
    geometry::Point,
    mono_font::{ascii::FONT_6X10, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

const TEXT_STYLE: MonoTextStyle<'static, BinaryColor> = MonoTextStyleBuilder::new()
    .font(&FONT_6X10)
    .text_color(BinaryColor::On)
    .background_color(BinaryColor::Off)
    .build();

pub trait DisplayDriver: DerefMut<Target = Self::DerefTarget> {
    type DerefTarget: DrawTarget<Color = BinaryColor>;

    async fn init(&mut self) -> Result<(), DisplayError> {
        Ok(())
    }

    fn flush(&mut self) -> Result<(), DisplayError>;
    fn clear_buffer(&mut self);
    async fn flush_async(&mut self) -> Result<(), DisplayError>;

    fn calculate_point(col: i32, row: i32) -> Point;

    async fn clear(&mut self) -> Result<(), DisplayError> {
        self.clear_buffer();
        self.flush_async().await
    }

    async fn update_text(&mut self, text: &str, point: Point) -> Result<(), DisplayError> {
        let _ = Text::with_baseline(text, point, TEXT_STYLE, Baseline::Top).draw(self.deref_mut());
        self.flush_async().await
    }
}
