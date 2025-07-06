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
    NoisePerlin,
}

/// Driver for controlling the RGB leds.
///
/// This trait essentially 'type alias' for blinksy's `DriverAsync` with `Color` is limited to
/// `LinearSrgb`.
pub trait RgbDriver: blinksy::driver::DriverAsync<Color = LinearSrgb> {}

impl<T: blinksy::driver::DriverAsync<Color = LinearSrgb>> RgbDriver for T {}
