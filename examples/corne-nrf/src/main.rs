#![no_std]
#![no_main]

mod keymap;

use core::panic::PanicInfo;

use embassy_executor::Spawner;
use embassy_nrf::{
    bind_interrupts,
    gpio::{Input, Level, Output, OutputDrive, Pull},
    usb::vbus_detect::SoftwareVbusDetect,
};
use rktk::{
    config::{CONST_CONFIG, new_rktk_opts},
    drivers::{Drivers, dummy},
    hooks::empty_hooks::create_empty_hooks,
    config::Hand,
    singleton,
};

use rktk_drivers_common::{
    keyscan::matrix::Matrix,
    usb::{CommonUsbDriverConfig, CommonUsbReporterBuilder, UsbDriverConfig},
};
use rktk_drivers_nrf::system::NrfSystemDriver;

bind_interrupts!(pub struct Irqs {
    USBD => embassy_nrf::usb::InterruptHandler<embassy_nrf::peripherals::USBD>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    // Output pins are arranged from left to right
    #[cfg(feature = "left")]
    let outputs = [
        Output::new(p.P1_11, Level::Low, OutputDrive::Standard), // COL5
        Output::new(p.P1_13, Level::Low, OutputDrive::Standard), // COL4
        Output::new(p.P1_15, Level::Low, OutputDrive::Standard), // COL3
        Output::new(p.P0_02, Level::Low, OutputDrive::Standard), // COL2
        Output::new(p.P0_29, Level::Low, OutputDrive::Standard), // COL1
        Output::new(p.P0_31, Level::Low, OutputDrive::Standard), // COL0
    ];

    #[cfg(feature = "right")]
    let outputs = [
        Output::new(p.P0_31, Level::Low, OutputDrive::Standard), // COL0
        Output::new(p.P0_29, Level::Low, OutputDrive::Standard), // COL1
        Output::new(p.P0_02, Level::Low, OutputDrive::Standard), // COL2
        Output::new(p.P1_15, Level::Low, OutputDrive::Standard), // COL3
        Output::new(p.P1_13, Level::Low, OutputDrive::Standard), // COL4
        Output::new(p.P1_11, Level::Low, OutputDrive::Standard), // COL5
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
                Input::new(p.P0_22, Pull::Down), // ROW0
                Input::new(p.P0_24, Pull::Down), // ROW1
                Input::new(p.P1_00, Pull::Down), // ROW2
                Input::new(p.P0_11, Pull::Down), // ROW3
            ],
            |row, col| Some((row, col)),
            None,
        ),
        system: NrfSystemDriver::new(None),
        mouse: dummy::mouse(),
        usb_builder: Some({
            let vbus = &*singleton!(SoftwareVbusDetect::new(true, true), SoftwareVbusDetect);
            let embassy_driver = embassy_nrf::usb::Driver::new(p.USBD, Irqs, vbus);
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
