//! Dummy keyboard to test dummy drivers.

#![no_std]
#![no_main]

mod keymap;

use core::panic::PanicInfo;

use embassy_executor::Spawner;
use rktk::{
    drivers::{
        Drivers,
        interface::keyscan::{KeyChangeEvent, KeyscanDriver},
    },
    hooks::empty_hooks::create_empty_hooks,
    none_driver,
};

use rktk_drivers_rp::system::RpSystemDriver;

pub struct DummyKeyscanDriver;
impl KeyscanDriver for DummyKeyscanDriver {
    async fn scan(&mut self, _cb: impl FnMut(KeyChangeEvent)) {
        let _: () = core::future::pending().await;
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _p = embassy_rp::init(Default::default());

    let drivers = Drivers {
        keyscan: DummyKeyscanDriver,
        system: RpSystemDriver,
        mouse_builder: none_driver!(MouseBuilder),
        usb_builder: none_driver!(UsbBuilder),
        display_builder: none_driver!(DisplayBuilder),
        split: none_driver!(Split),
        rgb: none_driver!(Rgb),
        ble_builder: none_driver!(BleBuilder),
        storage: none_driver!(Storage),
        debounce: none_driver!(Debounce),
        encoder: none_driver!(Encoder),
    };

    rktk::task::start(drivers, &keymap::KEYMAP, None, create_empty_hooks()).await;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
