use rktk::{
    config::keymap::Keymap,
    drivers::{Drivers, dummy},
    hooks::AllHooks,
};
use rktk_drivers_common::usb::{CommonUsbDriverConfig, CommonUsbReporterBuilder};

use crate::*;

pub async fn start_master(
    spawner: embassy_executor::Spawner,
    p: embassy_rp::Peripherals,
    hooks: impl AllHooks,
    keymap: &'static Keymap,
) {
    // create shared SPI bus
    // NOTE: This must be done as soon as possible, otherwise the SPI device will start acting strangely.
    let spi = create_spi!(p);

    let usb = {
        let embassy_driver = embassy_rp::usb::Driver::new(p.USB, Irqs);

        let mut config = rktk_drivers_common::usb::UsbDriverConfig::new(0xc0de, 0xcaff);

        config.manufacturer = Some("nazo6");
        config.product = Some("negL RP");
        config.serial_number = Some("12345678");
        config.max_power = 100;
        config.max_packet_size_0 = 64;
        config.supports_remote_wakeup = true;

        let opts = CommonUsbDriverConfig::new(embassy_driver, config);
        Some(CommonUsbReporterBuilder::new(opts))
    };

    let storage =
        rktk_drivers_rp::flash::init_storage::<_, { 4 * 1024 * 1024 }>(p.FLASH, p.DMA_CH3, Irqs);

    let drivers = Drivers {
        keyscan: driver_keyscan!(p, spi),
        system: driver_system!(),
        mouse: Some(driver_mouse!(p, spi)),
        usb_builder: usb,
        display: Some(driver_display!(p)),
        split: Some(driver_split!(p)),
        rgb: Some(driver_rgb!(p)),
        storage: Some(storage),
        ble_builder: dummy::ble_builder(),
        debounce: Some(driver_debounce!()),
        encoder: Some(driver_encoder!(p)),
    };

    rktk::task::start(spawner, drivers, hooks, neg_common::get_opts(keymap)).await;
}
