//! Just a dummy crate to ensure dummy drivers working

#![no_std]
#![no_main]

use core::panic::PanicInfo;

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::Flex;
use keymap::KEYMAP;
use rktk::{
    interface::{
        backlight::DummyBacklightDriver, ble::DummyBleDriver,
        double_tap::DummyDoubleTapResetDriver, mouse::DummyMouseDriverBuilder,
        split::DummySplitDriver, storage::DummyStorage, usb::DummyUsbDriver,
    },
    task::{Drivers, DummyDisplayDriverBuilder},
};
use rktk_drivers_rp2040::keyscan::duplex_matrix::create_duplex_matrix;

mod keymap;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let key_scanner = create_duplex_matrix::<'_, 5, 4, 5, 7>(
        [
            Flex::new(p.PIN_4),
            Flex::new(p.PIN_5),
            Flex::new(p.PIN_6),
            Flex::new(p.PIN_7),
            Flex::new(p.PIN_8),
        ],
        [
            Flex::new(p.PIN_29),
            Flex::new(p.PIN_28),
            Flex::new(p.PIN_27),
            Flex::new(p.PIN_26),
        ],
        (2, 6),
    );

    let drivers = Drivers {
        key_scanner,
        double_tap_reset: Option::<DummyDoubleTapResetDriver>::None,
        mouse_builder: Option::<DummyMouseDriverBuilder>::None,
        usb: Option::<DummyUsbDriver>::None,
        display_builder: Option::<DummyDisplayDriverBuilder>::None,
        split: Option::<DummySplitDriver>::None,
        backlight: Option::<DummyBacklightDriver>::None,
        ble: Option::<DummyBleDriver>::None,
        storage: Option::<&DummyStorage>::None,
    };

    rktk::task::start(drivers, KEYMAP).await;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}