use usbd_hid::descriptor;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct KeyboardReport {
    pub modifier: u8,
    pub reserved: u8,
    pub leds: u8,
    pub keycodes: [u8; 6],
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MouseReport {
    pub buttons: u8,
    pub x: i8,
    pub y: i8,
    pub wheel: i8,
    pub pan: i8,
}

impl From<KeyboardReport> for descriptor::KeyboardReport {
    fn from(value: KeyboardReport) -> Self {
        Self {
            modifier: value.modifier,
            reserved: value.reserved,
            leds: value.leds,
            keycodes: value.keycodes,
        }
    }
}
impl From<descriptor::KeyboardReport> for KeyboardReport {
    fn from(value: descriptor::KeyboardReport) -> Self {
        Self {
            modifier: value.modifier,
            reserved: value.reserved,
            leds: value.leds,
            keycodes: value.keycodes,
        }
    }
}

impl From<MouseReport> for descriptor::MouseReport {
    fn from(value: MouseReport) -> Self {
        Self {
            buttons: value.buttons,
            x: value.x,
            y: value.y,
            wheel: value.wheel,
            pan: value.pan,
        }
    }
}

impl From<descriptor::MouseReport> for MouseReport {
    fn from(value: descriptor::MouseReport) -> Self {
        Self {
            buttons: value.buttons,
            x: value.x,
            y: value.y,
            wheel: value.wheel,
            pan: value.pan,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum DongleData {
    Keyboard(KeyboardReport),
    Mouse(MouseReport),
}

pub trait DongleDriver {
    type Error: core::fmt::Debug;

    async fn recv(&mut self) -> Result<DongleData, Self::Error>;
}
