//! RGBBB driver type

// TODO: Split backlight and underglow

use serde::{Deserialize, Serialize};
use smart_leds::RGB8;

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub enum RgbCommand {
    Start(RgbMode),
    Reset,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub enum RgbMode {
    Rainbow,
    Blink,
    // Color (rgb)
    SolidColor(u8, u8, u8),
}

/// Driver for controlling the RGB leds.
pub trait RgbDriver: 'static {
    type Error: core::error::Error;

    /// Write provided colors to leds.
    async fn write<const N: usize>(&mut self, colors: &[RGB8; N]) -> Result<(), Self::Error>;
}
