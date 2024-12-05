use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

use crate::state::config::Output;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransparentReport {
    pub flash_clear: bool,
    pub ble_bond_clear: bool,
    pub output: Output,
    pub bootloader: bool,
}

/// Information to be communicated to the outside as a result of a state change
#[derive(Debug, PartialEq)]
pub struct StateReport {
    pub keyboard_report: Option<KeyboardReport>,
    pub mouse_report: Option<MouseReport>,
    pub media_keyboard_report: Option<MediaKeyboardReport>,
    pub transparent_report: TransparentReport,
    pub highest_layer: u8,
}

/// Represents a key event.
///
/// Used generically to indicate that the state of a physical key has changed
#[derive(Debug)]
pub struct KeyChangeEvent {
    pub col: u8,
    pub row: u8,
    pub pressed: bool,
}

/// Represents the direction of an encoder
#[derive(Debug)]
pub enum EncoderDirection {
    Clockwise,
    CounterClockwise,
}

#[derive(Debug)]
pub enum Event {
    Key(KeyChangeEvent),
    Mouse((i8, i8)),
    Encoder((u8, EncoderDirection)),
    None,
}
