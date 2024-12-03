#![no_std]
#![no_main]

mod keymap;

use core::panic::PanicInfo;

use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use rktk::{
    config::static_config::KEYBOARD,
    drivers::{interface::keyscan::Hand, Drivers},
    hooks::empty_hooks::create_empty_hooks,
    none_driver,
};

use keymap::KEY_CONFIG;
use rktk_drivers_common::keyscan::{matrix::Matrix, HandDetector};
use rktk_drivers_rp::system::RpSystemDriver;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let drivers = Drivers {
        keyscan: Matrix::<_, _, 6, 4, { KEYBOARD.cols as usize }, { KEYBOARD.rows as usize }>::new(
            [
                Output::new(p.PIN_20, Level::Low),
                Output::new(p.PIN_19, Level::Low),
                Output::new(p.PIN_18, Level::Low),
                Output::new(p.PIN_17, Level::Low),
                Output::new(p.PIN_16, Level::Low),
                Output::new(p.PIN_15, Level::Low),
            ],
            [
                Input::new(p.PIN_7, Pull::Down),
                Input::new(p.PIN_8, Pull::Down),
                Input::new(p.PIN_9, Pull::Down),
                Input::new(p.PIN_10, Pull::Down),
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

    rktk::task::start(drivers, KEY_CONFIG, create_empty_hooks()).await;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
