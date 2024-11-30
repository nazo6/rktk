//! Dummy keyboard to test dummy drivers.

#![no_std]
#![no_main]

mod keymap;

use core::panic::PanicInfo;

use embassy_executor::Spawner;
use rktk::{
    hooks::create_empty_hooks,
    interface::{
        backlight::DummyBacklightDriver, ble::DummyBleDriverBuilder, debounce::DummyDebounceDriver,
        display::DummyDisplayDriverBuilder, double_tap::DummyDoubleTapResetDriver,
        encoder::DummyEncoderDriver, keyscan::DummyKeyscanDriver, mouse::DummyMouseDriverBuilder,
        split::DummySplitDriver, storage::DummyStorageDriver, usb::DummyUsbDriverBuilder,
    },
    task::Drivers,
};

use keymap::KEY_CONFIG;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _p = embassy_rp::init(Default::default());

    let drivers = Drivers {
        keyscan: DummyKeyscanDriver,
        double_tap_reset: Option::<DummyDoubleTapResetDriver>::None,
        mouse_builder: Option::<DummyMouseDriverBuilder>::None,
        usb_builder: Option::<DummyUsbDriverBuilder>::None,
        display_builder: Option::<DummyDisplayDriverBuilder>::None,
        split: Option::<DummySplitDriver>::None,
        backlight: Option::<DummyBacklightDriver>::None,
        ble_builder: Option::<DummyBleDriverBuilder>::None,
        storage: Option::<DummyStorageDriver>::None,
        debounce: Option::<DummyDebounceDriver>::None,
        encoder: Option::<DummyEncoderDriver>::None,
    };

    rktk::task::start(drivers, KEY_CONFIG, create_empty_hooks()).await;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
