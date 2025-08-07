//! Dummy keyboard to test dummy drivers.

#![no_std]
#![no_main]

mod keymap;

use core::panic::PanicInfo;

use embassy_executor::Spawner;
use rktk::{
    config::new_rktk_opts,
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
async fn main(spawner: Spawner) {
    let _p = embassy_rp::init(Default::default());

    let drivers = Drivers {
        keyscan: DummyKeyscanDriver,
        system: RpSystemDriver,
        mouse: dummy::mouse(),
        usb_builder: dummy::usb_builder(),
        display: dummy::display(),
        split: dummy::split(),
        rgb: dummy::rgb(),
        ble_builder: dummy::ble_builder(),
        storage: dummy::storage(),
        debounce: dummy::debounce(),
        encoder: dummy::encoder(),
    };

    rktk::task::start(
        spawner,
        drivers,
        create_empty_hooks(),
        new_rktk_opts(&keymap::KEYMAP, None),
    )
    .await;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
