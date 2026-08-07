use embassy_rp::Peripherals;
use rktk::{
    config::keymap::Keymap,
    drivers::{Drivers, dummy},
    hooks::AllHooks,
};

use crate::*;

const EMPTY_KM: Keymap = Keymap::const_default();

pub async fn start_slave(spawner: embassy_executor::Spawner, p: Peripherals, hooks: impl AllHooks) {
    let spi = create_spi!(p);

    let drivers = Drivers {
        keyscan: driver_keyscan!(p, spi),
        system: driver_system!(),
        mouse: Some(driver_mouse!(p, spi)),
        usb_builder: dummy::usb_builder(),
        display: Some(driver_display!(p)),
        split: Some(driver_split!(p)),
        rgb: Some(driver_rgb!(p)),
        storage: dummy::storage(),
        ble_builder: dummy::ble_builder(),
        debounce: Some(driver_debounce!()),
        encoder: Some(driver_encoder!(p)),
    };
    rktk::task::start(spawner, drivers, hooks, neg_common::get_opts(&EMPTY_KM)).await;
}
