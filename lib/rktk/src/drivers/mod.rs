//! Drivers for the keyboard.

use interface::{
    ble::BleDriverBuilder, display::DisplayDriverBuilder, mouse::MouseDriverBuilder,
    split::SplitDriverBuilder, system::SystemDriver, usb::UsbDriverBuilder,
};

use crate::drivers::interface::{
    debounce::DebounceDriver, encoder::EncoderDriver, keyscan::KeyscanDriver, rgb::RgbDriver,
    storage::StorageDriver,
};

pub mod dummy;
pub mod interface;

/// All drivers required to run the keyboard.
///
/// Only the `key_scanner` and `usb` drivers are required.
/// For other drivers, if the value is None, it will be handled appropriately.
pub struct Drivers<
    System: SystemDriver,
    KeyScan: KeyscanDriver,
    Debounce: DebounceDriver,
    Encoder: EncoderDriver,
    Rgb: RgbDriver,
    Storage: StorageDriver,
    Split: SplitDriverBuilder,
    Ble: BleDriverBuilder,
    Usb: UsbDriverBuilder,
    Display: DisplayDriverBuilder,
    Mouse: MouseDriverBuilder,
> {
    pub system: System,
    pub keyscan: KeyScan,
    pub debounce: Option<Debounce>,
    pub encoder: Option<Encoder>,
    pub rgb: Option<Rgb>,
    pub storage: Option<Storage>,
    pub split_builder: Option<Split>,
    pub ble_builder: Option<Ble>,
    pub usb_builder: Option<Usb>,
    pub mouse_builder: Option<Mouse>,
    pub display_builder: Option<Display>,
}
