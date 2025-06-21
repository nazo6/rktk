//! RGBBB driver type

// TODO: Split backlight and underglow

pub use blinksy::color::{ColorCorrection, LinearSrgb};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RgbCommand {
    Start(RgbMode),
    Reset,
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RgbMode {
    Rainbow,
    Blink,
    // Color (rgb)
    SolidColor(u8, u8, u8),
}

/// Driver for controlling the RGB leds.
///
/// Basically, this is just async version trait of [`blinksy::driver::Driver`]. But color is
/// limited to LinearSrgb to avoid complexity.
/// TODO: When blinksy implements async drivers, remove this trait and use the blinksy one
/// directly.
pub trait RgbDriver: 'static {
    type Error: super::Error;

    // Required method
    async fn write<I: IntoIterator<Item = LinearSrgb>>(
        &mut self,
        pixels: I,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Result<(), Self::Error>;
}
