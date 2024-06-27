#![no_std]
#![no_main]

use core::panic::PanicInfo;

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::Flex;
use rktk_drivers_rp2040::{
    double_tap::DoubleTapResetRp,
    interrupts::Irqs,
    keyscan::duplex_matrix::create_duplex_matrix,
    mouse::pmw3360::create_pmw3360,
    usb::{UsbConfig, UsbDriver, UsbUserOpts},
};

mod keymap;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let dtr = DoubleTapResetRp;

    let Ok(ball) = create_pmw3360(
        p.SPI0, p.PIN_22, p.PIN_23, p.PIN_20, p.DMA_CH0, p.DMA_CH1, p.PIN_21,
    )
    .await
    else {
        panic!("Failed to create PMW3360");
    };

    let rows = [
        Flex::new(p.PIN_4),
        Flex::new(p.PIN_5),
        Flex::new(p.PIN_6),
        Flex::new(p.PIN_7),
        Flex::new(p.PIN_8),
    ];
    let cols = [
        Flex::new(p.PIN_29),
        Flex::new(p.PIN_28),
        Flex::new(p.PIN_27),
        Flex::new(p.PIN_26),
    ];

    let key_scanner = create_duplex_matrix::<'_, 5, 4, 5, 7>(rows, cols, (2, 6));

    let mut config = UsbConfig::new(0xc0de, 0xcafe);
    config.manufacturer = Some("Yowkees/nazo6");
    config.product = Some("keyball");
    config.serial_number = Some("12345678");
    config.max_power = 100;
    config.max_packet_size_0 = 64;
    config.supports_remote_wakeup = true;
    let usb_opts = UsbUserOpts {
        config,
        mouse_poll_interval: 5,
        kb_poll_interval: 5,
    };
    let driver = embassy_rp::usb::Driver::new(p.USB, Irqs);

    let usb = UsbDriver::create_and_start(usb_opts, driver).await;

    rktk::task::start(Some(dtr), key_scanner, Some(ball), usb, keymap::KEYMAP).await;
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}
