//! RGBBB driver type

// TODO: Split backlight and underglow

pub use blinksy::color::ColorCorrection;
use blinksy::color::LinearSrgb;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Rgb8 {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
impl Rgb8 {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

impl From<LinearSrgb> for Rgb8 {
    fn from(color: LinearSrgb) -> Self {
        Self {
            red: (color.red * 255.0) as u8,
            green: (color.green * 255.0) as u8,
            blue: (color.blue * 255.0) as u8,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RgbCommand {
    Start(RgbMode),
    Reset,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RgbMode {
    Off,
    /// Color (rgb)
    SolidColor(u8, u8, u8),
    Pattern(RgbPattern),
    Custom,
}

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
    async fn write<I: IntoIterator<Item = Rgb8>>(
        &mut self,
        pixels: I,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Result<(), Self::Error>;
}
