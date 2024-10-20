//! Just a dummy crate to ensure dummy drivers working

#![no_std]
#![no_main]

use core::panic::PanicInfo;

use defmt_rtt as _;
use embassy_executor::Spawner;
use keyball_common::KEY_CONFIG;
use rktk::{
    hooks::create_empty_hooks,
    interface::{
        backlight::DummyBacklightDriver, ble::DummyBleDriver, display::DummyDisplayDriverBuilder,
        double_tap::DummyDoubleTapResetDriver, keyscan::DummyKeyscanDriver,
        mouse::DummyMouseDriverBuilder, split::DummySplitDriver, storage::DummyStorageDriver,
        usb::DummyUsbDriverBuilder,
    },
    task::Drivers,
};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _p = embassy_rp::init(Default::default());

    let drivers = Drivers {
        key_scanner: DummyKeyscanDriver,
        double_tap_reset: Option::<DummyDoubleTapResetDriver>::None,
        mouse_builder: Option::<DummyMouseDriverBuilder>::None,
        usb_builder: Option::<DummyUsbDriverBuilder>::None,
        display_builder: Option::<DummyDisplayDriverBuilder>::None,
        split: DummySplitDriver,
        backlight: Option::<DummyBacklightDriver>::None,
        ble: Option::<DummyBleDriver>::None,
        storage: Option::<DummyStorageDriver>::None,
    };

    rktk::task::start(drivers, KEY_CONFIG, create_empty_hooks()).await;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
