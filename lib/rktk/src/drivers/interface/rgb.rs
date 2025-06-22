//! RGBBB driver type

// TODO: Split backlight and underglow

pub use blinksy::color::{ColorCorrection, LedChannels, LedRgb, LinearSrgb};
use serde::{Deserialize, Serialize};

/// Commands for controlling RGB LEDs.
///
/// This value can be send using [`crate::hooks::channels::rgb::rgb_sender`].
/// In master side, command sent from above channel will also be sent to slave side.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RgbCommand {
    /// Set RGB mode and start it
    Start(RgbMode),
    /// Set brightness
    ///
    /// Range: 0.0 to 1.0
    Brightness(f32),
    BrightnessDelta(f32),
    /// Reset RGB state and restart current RgbMode
    Reset,
}

/// RGB mode for controlling RGB LEDs.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RgbMode {
    /// Turn off RGB
    Off,
    /// Set solid color
    ///
    /// Value range: 0 to 255
    /// (Red, Green, Blue)
    SolidColor(u8, u8, u8),
    /// Set built-in RGB pattern
    Pattern(RgbPattern),
    /// Call user-defined RGB hook
    Custom,
}

/// Built-in RGB patterns.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RgbPattern {
    Rainbow(f32, f32),
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
    async fn write<I: IntoIterator<Item = LedRgb<u8>>>(
        &mut self,
        pixels: I,
    ) -> Result<(), Self::Error>;
}
