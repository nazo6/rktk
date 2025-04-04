use postcard::experimental::max_size::MaxSize;
use serde::{Deserialize, Serialize};
use usbd_hid::descriptor;

#[derive(Debug, Serialize, Deserialize, MaxSize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct KeyboardReport {
    pub modifier: u8,
    pub keycodes: heapless::Vec<u8, 6>,
}

#[derive(Debug, Serialize, Deserialize, MaxSize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MouseReport {
    pub buttons: u8,
    pub x: i8,
    pub y: i8,
    pub wheel: i8,
    pub pan: i8,
}

#[derive(Debug, Serialize, Deserialize, MaxSize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MediaKeyboardReport {
    pub usage_id: u16,
}

impl From<KeyboardReport> for descriptor::KeyboardReport {
    fn from(value: KeyboardReport) -> Self {
        let mut keycodes = value.keycodes;
        keycodes.resize_default(6).unwrap();

        Self {
            modifier: value.modifier,
            reserved: 0,
            leds: 0,
            keycodes: keycodes.into_array().unwrap(),
        }
    }
}
impl From<descriptor::KeyboardReport> for KeyboardReport {
    fn from(value: descriptor::KeyboardReport) -> Self {
        Self {
            modifier: value.modifier,
            keycodes: heapless::Vec::from_slice(&value.keycodes).unwrap(),
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

impl From<MediaKeyboardReport> for descriptor::MediaKeyboardReport {
    fn from(value: MediaKeyboardReport) -> Self {
        Self {
            usage_id: value.usage_id,
        }
    }
}
impl From<descriptor::MediaKeyboardReport> for MediaKeyboardReport {
    fn from(value: descriptor::MediaKeyboardReport) -> Self {
        Self {
            usage_id: value.usage_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, MaxSize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DongleData {
    Keyboard(KeyboardReport),
    Mouse(MouseReport),
    MediaKeyboard(MediaKeyboardReport),
}

pub trait DongleDriver {
    type Error: core::fmt::Debug;

    async fn recv(&mut self) -> Result<DongleData, Self::Error>;
}

super::generate_builder!(DongleDriver);
