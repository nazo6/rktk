#![no_std]
#![no_main]

mod keymap;

use core::panic::PanicInfo;

use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    gpio::{Input, Level, Output, Pull},
};
use rktk::{
    config::{CONST_CONFIG, new_rktk_opts},
    drivers::{Drivers, dummy},
    hooks::empty_hooks::create_empty_hooks,
    config::Hand,
};

use rktk_drivers_common::{
    keyscan::matrix::Matrix,
    usb::{CommonUsbDriverConfig, CommonUsbReporterBuilder, UsbDriverConfig},
};
use rktk_drivers_rp::system::RpSystemDriver;

bind_interrupts!(pub struct Irqs {
    USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<embassy_rp::peripherals::USB>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Output pins are arranged from left to right
    #[cfg(feature = "left")]
    let outputs = [
        Output::new(p.PIN_15, Level::Low), // COL5
        Output::new(p.PIN_16, Level::Low), // COL4
        Output::new(p.PIN_17, Level::Low), // COL3
        Output::new(p.PIN_18, Level::Low), // COL2
        Output::new(p.PIN_19, Level::Low), // COL1
        Output::new(p.PIN_20, Level::Low), // COL0
    ];

    #[cfg(feature = "right")]
    let outputs = [
        Output::new(p.PIN_20, Level::Low), // COL0
        Output::new(p.PIN_19, Level::Low), // COL1
        Output::new(p.PIN_18, Level::Low), // COL2
        Output::new(p.PIN_17, Level::Low), // COL3
        Output::new(p.PIN_16, Level::Low), // COL4
        Output::new(p.PIN_15, Level::Low), // COL5
    ];

    let drivers = Drivers {
        keyscan: Matrix::<
            _,
            _,
            _,
            6,
            4,
            { CONST_CONFIG.keyboard.rows as usize },
            { CONST_CONFIG.keyboard.cols as usize },
        >::new(
            outputs,
            [
                Input::new(p.PIN_7, Pull::Down),  // ROW0
                Input::new(p.PIN_8, Pull::Down),  // ROW1
                Input::new(p.PIN_9, Pull::Down),  // ROW2
                Input::new(p.PIN_10, Pull::Down), // ROW3
            ],
            |row, col| Some((row, col)),
            None,
        ),
        system: RpSystemDriver,
        mouse: dummy::mouse(),
        usb_builder: Some({
            let embassy_driver = embassy_rp::usb::Driver::new(p.USB, Irqs);
            let mut driver_config = UsbDriverConfig::new(0xc0de, 0xcafe);
            driver_config.product = Some("corne");
            let opts = CommonUsbDriverConfig::new(embassy_driver, driver_config);

            CommonUsbReporterBuilder::new(opts)
        }),
        display: dummy::display(),
        split: dummy::split(),
        rgb: dummy::rgb(),
        ble_builder: dummy::ble_builder(),
        storage: dummy::storage(),
        debounce: dummy::debounce(),
        encoder: dummy::encoder(),
    };

    rktk::task::start(
        drivers,
        create_empty_hooks(),
        new_rktk_opts(&keymap::KEYMAP, {
            #[cfg(feature = "left")]
            {
                Some(Hand::Left)
            }
            #[cfg(feature = "right")]
            {
                Some(Hand::Right)
            }
        }),
    )
    .await;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
