//! Drivers for the keyboard.

use interface::system::SystemDriver;

use crate::drivers::interface::{
    ble::BleDriver, debounce::DebounceDriver, display::DisplayDriver, encoder::EncoderDriver,
    keyscan::KeyscanDriver, mouse::MouseDriver, rgb::RgbDriver, split::SplitDriver,
    storage::StorageDriver, usb::UsbDriver, DriverBuilder, DriverBuilderWithTask,
};

pub mod dummy;
pub mod interface;

/// Utility to pass `None` as a driver.
///
/// In Rust, you need to specify a type even when putting None into an Option value that uses generics.
/// This can be cumbersome when not using a driver, so we provide a driver for type annotations that cannot be constructed, such as [`dummy`].
/// This macro makes it easy to use the dummy driver.
#[macro_export]
macro_rules! none_driver {
    ($type:ident) => {
        Option::<$crate::drivers::dummy::$type>::None
    };
}

/// All drivers required to run the keyboard.
///
/// Only the `key_scanner` and `usb` drivers are required.
/// For other drivers, if the value is None, it will be handled appropriately.
pub struct Drivers<
    KeyScan: KeyscanDriver,
    Debounce: DebounceDriver,
    Encoder: EncoderDriver,
    Ble: BleDriver,
    Usb: UsbDriver,
    Split: SplitDriver,
    Rgb: RgbDriver,
    System: SystemDriver,
    Storage: StorageDriver,
    Mouse: MouseDriver,
    Display: DisplayDriver,
    MouseBuilder: DriverBuilder<Output = Mouse>,
    DisplayBuilder: DriverBuilder<Output = Display>,
    UsbBuilder: DriverBuilderWithTask<Driver = Usb>,
    BleBuilder: DriverBuilderWithTask<Driver = Ble>,
> {
    pub system: System,
    pub keyscan: KeyScan,
    pub debounce: Option<Debounce>,
    pub encoder: Option<Encoder>,
    pub split: Option<Split>,
    pub rgb: Option<Rgb>,
    pub storage: Option<Storage>,

    pub ble_builder: Option<BleBuilder>,
    pub usb_builder: Option<UsbBuilder>,
    pub mouse_builder: Option<MouseBuilder>,
    pub display_builder: Option<DisplayBuilder>,
}
