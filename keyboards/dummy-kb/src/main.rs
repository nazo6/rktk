//! Just a dummy crate to ensure dummy drivers working

#![no_std]
#![no_main]

mod keymap;

use core::panic::PanicInfo;

use embassy_executor::Spawner;
use rktk::{
    hooks::create_empty_hooks,
    interface::{
        debounce::NoopDebounceDriver, keyscan::DummyKeyscanDriver, split::DummySplitDriver,
    },
    task::drivers::Drivers,
};

use keymap::KEY_CONFIG;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _p = embassy_rp::init(Default::default());

    let drivers: Drivers<_, _, _> = Drivers::builder()
        .keyscan(DummyKeyscanDriver)
        .debounce(NoopDebounceDriver)
        .split(DummySplitDriver)
        .build();

    rktk::task::start(drivers, KEY_CONFIG, create_empty_hooks()).await;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
