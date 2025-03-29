#![no_std]
#![no_main]

mod keymap;

use core::panic::PanicInfo;

use embassy_executor::Spawner;
use embassy_nrf::gpio::{Input, Level, Output, OutputDrive, Pull};
use rktk::{
    config::constant::CONFIG,
    drivers::{Drivers, dummy},
    hooks::empty_hooks::create_empty_hooks,
    interface::Hand,
};

use rktk_drivers_common::keyscan::matrix::Matrix;
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
            _,
            6,
            4,
            { CONFIG.keyboard.rows as usize },
            { CONFIG.keyboard.cols as usize },
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
        mouse: dummy::mouse_builder(),
        usb_builder: dummy::usb_builder(),
        display: dummy::display_builder(),
        split: dummy::split_builder(),
        rgb: dummy::rgb(),
        ble_builder: dummy::ble_builder(),
        storage: dummy::storage(),
        debounce: dummy::debounce(),
        encoder: dummy::encoder(),
    };

    rktk::task::start(
        drivers,
        &keymap::KEYMAP,
        {
            #[cfg(feature = "left")]
            {
                Some(Hand::Left)
            }
            #[cfg(feature = "right")]
            {
                Some(Hand::Right)
            }
        },
        create_empty_hooks(),
    )
    .await;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
