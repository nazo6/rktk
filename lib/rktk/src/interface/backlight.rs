use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub enum LedControl {
    Start(LedAnimation),
    Reset,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub enum LedAnimation {
    Rainbow,
    Blink,
    // Color (rgb)
    SolidColor(u8, u8, u8),
}
