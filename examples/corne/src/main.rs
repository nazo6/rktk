#![no_std]
#![no_main]

mod keymap;

use core::panic::PanicInfo;

use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use rktk::{
    config::{Config, Keyboard, CONST_CONFIG},
    drivers::{interface::keyscan::Hand, Drivers},
    hooks::empty_hooks::create_empty_hooks,
    none_driver,
};

use rktk_drivers_common::keyscan::{matrix::Matrix, HandDetector};
use rktk_drivers_rp::system::RpSystemDriver;

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

    let drivers =
        Drivers {
            keyscan: Matrix::<
                _,
                _,
                6,
                4,
                { CONST_CONFIG.cols as usize },
                { CONST_CONFIG.rows as usize },
            >::new(
                outputs,
                [
                    Input::new(p.PIN_7, Pull::Down),  // ROW0
                    Input::new(p.PIN_8, Pull::Down),  // ROW1
                    Input::new(p.PIN_9, Pull::Down),  // ROW2
                    Input::new(p.PIN_10, Pull::Down), // ROW3
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
            ),
            system: RpSystemDriver,
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

    let config = Config {
        keyboard: Keyboard {
            name: "corne",
            ..Default::default()
        },
        ..Default::default()
    };

    rktk::task::start(drivers, keymap::KEYMAP, create_empty_hooks(), config).await;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
