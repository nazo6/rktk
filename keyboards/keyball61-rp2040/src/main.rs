#![no_std]
#![no_main]

use core::panic::PanicInfo;

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::{gpio::Flex, pio::Pio};
use keymap::KEYMAP;
use rktk::task::Drivers;
use rktk_drivers_rp2040::{
    display::ssd1306::create_ssd1306,
    double_tap::DoubleTapResetRp,
    keyscan::duplex_matrix::create_duplex_matrix,
    mouse::pmw3360::create_pmw3360,
    split::pio_half_duplex::PioHalfDuplexSplitDriver,
    usb::{new_usb, UsbConfig, UsbUserOpts},
};

mod keymap;

use embassy_rp::{
    bind_interrupts,
    peripherals::{I2C1, PIO0, USB},
};

bind_interrupts!(pub struct Irqs {
    USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
    I2C1_IRQ => embassy_rp::i2c::InterruptHandler<I2C1>;
    PIO0_IRQ_0 => embassy_rp::pio::InterruptHandler<PIO0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let dtr = DoubleTapResetRp;

    let display = create_ssd1306(
        p.I2C1,
        Irqs,
        p.PIN_2,
        p.PIN_3,
        ssd1306::size::DisplaySize128x32,
    )
    .expect("Failed to create SSD1306");

    let ball = create_pmw3360(
        p.SPI0, p.PIN_22, p.PIN_23, p.PIN_20, p.DMA_CH0, p.DMA_CH1, p.PIN_21,
    );

    let key_scanner = create_duplex_matrix::<'_, 5, 4, 5, 7>(
        [
            Flex::new(p.PIN_4),
            Flex::new(p.PIN_5),
            Flex::new(p.PIN_6),
            Flex::new(p.PIN_7),
            Flex::new(p.PIN_8),
        ],
        [
            Flex::new(p.PIN_29),
            Flex::new(p.PIN_28),
            Flex::new(p.PIN_27),
            Flex::new(p.PIN_26),
        ],
        (2, 6),
    );

    let usb = {
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

        new_usb(usb_opts, driver).await
    };

    let pio = Pio::new(p.PIO0, Irqs);
    let split = PioHalfDuplexSplitDriver::new(pio, p.PIN_1);

    let drivers = Drivers {
        key_scanner,
        double_tap_reset: Some(dtr),
        mouse: Some(ball),
        usb,
        display: Some(display),
        split,
    };

    rktk::task::start(drivers, KEYMAP).await;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
