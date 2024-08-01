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

/// Interface for display drivers.
///
/// TODO: Allow sync-only drivers?
/// TODO: Make text style configurable.
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

    fn update_text_sync(&mut self, text: &str, point: Point) -> Result<(), DisplayError> {
        let _ = Text::with_baseline(text, point, TEXT_STYLE, Baseline::Top).draw(self.deref_mut());
        self.flush()
    }
}

pub enum DummyDisplay {}
impl Dimensions for DummyDisplay {
    fn bounding_box(&self) -> embedded_graphics::primitives::Rectangle {
        unimplemented!()
    }
}
impl DrawTarget for DummyDisplay {
    type Color = BinaryColor;

    type Error = ();

    fn draw_iter<I>(&mut self, _pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        unimplemented!()
    }
}

/// Dummy driver that is only used to be given as a type argument.
pub enum DummyDisplayDriver {}
impl DisplayDriver for DummyDisplayDriver {
    type DerefTarget = DummyDisplay;
    fn flush(&mut self) -> Result<(), DisplayError> {
        unimplemented!()
    }
    fn clear_buffer(&mut self) {
        unimplemented!()
    }
    fn calculate_point(_col: i32, _row: i32) -> Point {
        unimplemented!()
    }
    async fn flush_async(&mut self) -> Result<(), DisplayError> {
        unimplemented!()
    }
}

impl core::ops::Deref for DummyDisplayDriver {
    type Target = DummyDisplay;
    fn deref(&self) -> &Self::Target {
        unimplemented!()
    }
}
impl core::ops::DerefMut for DummyDisplayDriver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unimplemented!()
    }
}
