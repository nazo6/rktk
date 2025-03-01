#![no_std]
#![no_main]

mod keymap;

use core::panic::PanicInfo;

use embassy_executor::Spawner;
use embassy_nrf::gpio::{Input, Level, Output, OutputDrive, Pull};
use rktk::{
    config::constant::CONFIG,
    drivers::{interface::keyscan::Hand, Drivers},
    hooks::empty_hooks::create_empty_hooks,
    none_driver,
};

use rktk_drivers_common::keyscan::{matrix::Matrix, HandDetector};
use rktk_drivers_nrf::system::NrfSystemDriver;

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
            6,
            4,
            { CONFIG.keyboard.cols as usize },
            { CONFIG.keyboard.rows as usize },
        >::new(
            outputs,
            [
                Input::new(p.P0_22, Pull::Down), // ROW0
                Input::new(p.P0_24, Pull::Down), // ROW1
                Input::new(p.P1_00, Pull::Down), // ROW2
                Input::new(p.P0_11, Pull::Down), // ROW3
            ],
            HandDetector::Constant({
                #[cfg(feature = "left")]
                {
                    Hand::Left
                }
                #[cfg(feature = "right")]
                {
                    Hand::Right
                }
            }),
            |row, col| Some((row, col)),
            None,
        ),
        system: NrfSystemDriver::new(None),
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

    rktk::task::start(drivers, keymap::KEYMAP, create_empty_hooks()).await;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
