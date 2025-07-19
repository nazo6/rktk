use rktk::{
    config::keymap::Keymap,
    drivers::{Drivers, dummy},
    hooks::{Hooks, interface::*},
};

#[cfg(feature = "sd")]
use crate::common::init_sd;
use crate::{common::init_peri, *};

const EMPTY_KM: Keymap = Keymap::const_default();

pub async fn start_slave(
    hooks: Hooks<impl CommonHooks, impl MasterHooks, impl SlaveHooks, impl RgbHooks>,
) {
    let p = init_peri();

    #[cfg(feature = "sd")]
    let _ = init_sd().await;

    let spi = create_spi!(p);

    let drivers = Drivers {
        keyscan: driver_keyscan!(p, spi),
        system: driver_system!(p),
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
    rktk::task::start(drivers, hooks, misc::get_opts(&EMPTY_KM)).await;
}
