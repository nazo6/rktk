use display_interface::DisplayError;
use embedded_graphics::{
    geometry::Point,
    mono_font::MonoTextStyle,
    prelude::*,
    text::{Baseline, Text},
};

/// Interface for display drivers.
///
/// TODO: Allow sync-only drivers?
pub trait DisplayDriver: DrawTarget + Sized {
    const MAX_TEXT_WIDTH: usize;
    const TEXT_STYLE: MonoTextStyle<'static, Self::Color>;

    fn clear_buffer(&mut self);
    async fn flush(&mut self) -> Result<(), DisplayError>;

    fn calculate_point(col: i32, row: i32) -> Point;

    async fn clear_flush(&mut self) -> Result<(), DisplayError> {
        self.clear_buffer();
        self.flush().await
    }

    fn draw_text(&mut self, text: &str, point: Point) {
        let _ = Text::with_baseline(text, point, Self::TEXT_STYLE, Baseline::Top).draw(self);
    }

    async fn update_text(&mut self, text: &str, point: Point) -> Result<(), DisplayError> {
        let _ = Text::with_baseline(text, point, Self::TEXT_STYLE, Baseline::Top).draw(self);
        self.flush().await
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

        let _ = self.flush().await;
    }

    /// Sets brightness of the display.
    ///
    /// 0 is off, 255 is full brightness.
    async fn set_brightness(&mut self, _brightness: u8) -> Result<(), DisplayError> {
        Err(DisplayError::DataFormatNotImplemented)
    }
}
