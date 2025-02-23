use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

use super::Output;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransparentReport {
    pub flash_clear: bool,
    pub ble_bond_clear: bool,
    pub output: Output,
    pub bootloader: bool,
    pub power_off: bool,
}

/// Information to be communicated to the outside as a result of a state change
#[derive(Debug, PartialEq, Clone)]
pub struct StateReport {
    pub keyboard_report: Option<KeyboardReport>,
    pub mouse_report: Option<MouseReport>,
    pub media_keyboard_report: Option<MediaKeyboardReport>,
    pub transparent_report: TransparentReport,
    pub highest_layer: u8,
}
