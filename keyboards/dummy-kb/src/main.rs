//! Dummy keyboard to test dummy drivers.

#![no_std]
#![no_main]

mod keymap;

use core::panic::PanicInfo;

use embassy_executor::Spawner;
use rktk::{
    drivers::{
        interface::keyscan::{Hand, KeyscanDriver},
        Drivers,
    },
    hooks::create_empty_hooks,
    keymanager::state::KeyChangeEvent,
    none_driver,
};

use keymap::KEY_CONFIG;

pub struct DummyKeyscanDriver;
impl KeyscanDriver for DummyKeyscanDriver {
    async fn scan(&mut self, _cb: impl FnMut(KeyChangeEvent)) {
        let _: () = core::future::pending().await;
    }
    async fn current_hand(&mut self) -> Hand {
        Hand::Right
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _p = embassy_rp::init(Default::default());

    let drivers = Drivers {
        keyscan: DummyKeyscanDriver,
        double_tap_reset: none_driver!(DoubleTapReset),
        mouse_builder: none_driver!(MouseBuilder),
        usb_builder: none_driver!(UsbBuilder),
        display_builder: none_driver!(DisplayBuilder),
        split: none_driver!(Split),
        backlight: none_driver!(Backlight),
        ble_builder: none_driver!(BleBuilder),
        storage: none_driver!(Storage),
        debounce: none_driver!(Debounce),
        encoder: none_driver!(Encoder),
    };

    rktk::task::start(drivers, KEY_CONFIG, create_empty_hooks()).await;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
