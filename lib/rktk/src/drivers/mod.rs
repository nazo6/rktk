//! Drivers for the keyboard.

use interface::{
    display::DisplayDriver, mouse::MouseDriver, split::SplitDriver, system::SystemDriver,
    usb::UsbReporterDriverBuilder, wireless::WirelessReporterDriverBuilder,
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
    Split: SplitDriver,
    Ble: WirelessReporterDriverBuilder,
    Usb: UsbReporterDriverBuilder,
    Display: DisplayDriver,
    Mouse: MouseDriver,
> {
    pub system: System,
    pub keyscan: KeyScan,
    pub debounce: Option<Debounce>,
    pub encoder: Option<Encoder>,
    pub rgb: Option<Rgb>,
    pub storage: Option<Storage>,
    pub split: Option<Split>,
    pub ble_builder: Option<Ble>,
    pub usb_builder: Option<Usb>,
    pub mouse: Option<Mouse>,
    pub display: Option<Display>,
}
