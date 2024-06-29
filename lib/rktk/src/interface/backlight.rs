use serde::{Deserialize, Serialize};
use smart_leds::RGB8;

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub enum BacklightCtrl {
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
    async fn write<const N: usize>(&mut self, colors: &[RGB8; N]);
}

pub struct DummyBacklightDriver;
impl BacklightDriver for DummyBacklightDriver {
    async fn write<const N: usize>(&mut self, _colors: &[RGB8; N]) {}
}
