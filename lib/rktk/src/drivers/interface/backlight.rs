use serde::{Deserialize, Serialize};
use smart_leds::RGB8;

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub enum BacklightCommand {
    Start(BacklightMode),
    Reset,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub enum BacklightMode {
    Rainbow,
    Blink,
    // Color (rgb)
    SolidColor(u8, u8, u8),
}

pub trait BacklightDriver {
    type Error: core::error::Error;

    async fn write<const N: usize>(&mut self, colors: &[RGB8; N]) -> Result<(), Self::Error>;
}
