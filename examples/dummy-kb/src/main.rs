//! Dummy keyboard to test dummy drivers.

#![no_std]
#![no_main]

mod keymap;

use core::panic::PanicInfo;

use embassy_executor::Spawner;
use rktk::{
    drivers::{
        Drivers, dummy,
        interface::keyscan::{KeyChangeEvent, KeyscanDriver},
    },
    hooks::empty_hooks::create_empty_hooks,
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
        mouse: dummy::mouse_builder(),
        usb_builder: dummy::usb_builder(),
        display: dummy::display_builder(),
        split_builder: dummy::split_builder(),
        rgb: dummy::rgb(),
        ble_builder: dummy::ble_builder(),
        storage: dummy::storage(),
        debounce: dummy::debounce(),
        encoder: dummy::encoder(),
    };

    rktk::task::start(drivers, &keymap::KEYMAP, None, create_empty_hooks()).await;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
