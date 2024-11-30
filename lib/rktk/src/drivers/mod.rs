use crate::interface::{
    backlight::BacklightDriver, ble::BleDriver, debounce::DebounceDriver, display::DisplayDriver,
    double_tap::DoubleTapResetDriver, encoder::EncoderDriver, keyscan::KeyscanDriver,
    mouse::MouseDriver, split::SplitDriver, storage::StorageDriver, usb::UsbDriver, DriverBuilder,
    DriverBuilderWithTask,
};

pub mod dummy;

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
    Backlight: BacklightDriver,
    DoubleTapReset: DoubleTapResetDriver,
    Storage: StorageDriver,
    Mouse: MouseDriver,
    Display: DisplayDriver,
    MouseBuilder: DriverBuilder<Output = Mouse>,
    DisplayBuilder: DriverBuilder<Output = Display>,
    UsbBuilder: DriverBuilderWithTask<Driver = Usb>,
    BleBuilder: DriverBuilderWithTask<Driver = Ble>,
> {
    pub double_tap_reset: Option<DoubleTapReset>,
    pub keyscan: KeyScan,
    pub debounce: Option<Debounce>,
    pub encoder: Option<Encoder>,
    pub split: Option<Split>,
    pub backlight: Option<Backlight>,
    pub storage: Option<Storage>,

    pub ble_builder: Option<BleBuilder>,
    pub usb_builder: Option<UsbBuilder>,
    pub mouse_builder: Option<MouseBuilder>,
    pub display_builder: Option<DisplayBuilder>,
}
