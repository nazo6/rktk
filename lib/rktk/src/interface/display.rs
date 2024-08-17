use display_interface::DisplayError;
use embedded_graphics::{
    geometry::Point,
    mono_font::{ascii::FONT_6X10, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

use super::DriverBuilder;

/// Interface for display drivers.
///
/// TODO: Allow sync-only drivers?
pub trait DisplayDriver: DrawTarget + Sized {
    const MAX_TEXT_WIDTH: usize;
    const TEXT_STYLE: MonoTextStyle<'static, Self::Color>;

    fn flush(&mut self) -> Result<(), DisplayError>;
    fn clear_buffer(&mut self);
    async fn flush_async(&mut self) -> Result<(), DisplayError>;

    fn calculate_point(col: i32, row: i32) -> Point;

    async fn clear_flush(&mut self) -> Result<(), DisplayError> {
        self.clear_buffer();
        self.flush_async().await
    }

    fn draw_text(&mut self, text: &str, point: Point) {
        let _ = Text::with_baseline(text, point, Self::TEXT_STYLE, Baseline::Top).draw(self);
    }

    async fn update_text(&mut self, text: &str, point: Point) -> Result<(), DisplayError> {
        let _ = Text::with_baseline(text, point, Self::TEXT_STYLE, Baseline::Top).draw(self);
        self.flush_async().await
    }

    fn update_text_sync(&mut self, text: &str, point: Point) -> Result<(), DisplayError> {
        let _ = Text::with_baseline(text, point, Self::TEXT_STYLE, Baseline::Top).draw(self);
        self.flush()
    }

    /// Print a message on the display.
    ///
    /// In default implementation, the message is split into two lines if it is longer than the width.
    async fn print_message(&mut self, msg: &str) {
        self.draw_text("                        ", Self::calculate_point(1, 2));
        self.draw_text("                        ", Self::calculate_point(1, 3));

        if let Some((l1, l2)) = msg.split_at_checked(Self::MAX_TEXT_WIDTH) {
            self.draw_text(l1, Self::calculate_point(1, 2));
            self.draw_text(l2, Self::calculate_point(1, 3));
        } else {
            self.draw_text(msg, Self::calculate_point(1, 2));
        }

        let _ = self.flush_async().await;
    }
}

pub enum DummyDisplayDriver {}
impl Dimensions for DummyDisplayDriver {
    fn bounding_box(&self) -> embedded_graphics::primitives::Rectangle {
        unimplemented!()
    }
}
impl DrawTarget for DummyDisplayDriver {
    type Color = BinaryColor;

    type Error = ();

    fn draw_iter<I>(&mut self, _pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        unimplemented!()
    }
}
impl DisplayDriver for DummyDisplayDriver {
    const TEXT_STYLE: MonoTextStyle<'static, BinaryColor> = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .background_color(BinaryColor::Off)
        .build();
    const MAX_TEXT_WIDTH: usize = 10;

    fn flush(&mut self) -> Result<(), DisplayError> {
        unreachable!()
    }
    fn clear_buffer(&mut self) {
        unreachable!()
    }
    async fn flush_async(&mut self) -> Result<(), DisplayError> {
        unreachable!()
    }
    fn calculate_point(_col: i32, _row: i32) -> Point {
        unreachable!()
    }
}

pub enum DummyDisplayDriverBuilder {}
impl DriverBuilder for DummyDisplayDriverBuilder {
    type Output = DummyDisplayDriver;

    type Error = ();

    async fn build(self) -> Result<Self::Output, Self::Error> {
        unreachable!()
    }
}
