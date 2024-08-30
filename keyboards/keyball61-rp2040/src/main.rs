#![no_std]
#![no_main]

use core::panic::PanicInfo;

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::{flash::Flash, gpio::Flex, peripherals::PIO1, pio::Pio};
use rktk::{interface::ble::DummyBleDriver, task::Drivers};
use rktk_drivers_rp2040::{
    backlight::ws2812_pio::Ws2812Pio,
    display::ssd1306::create_ssd1306,
    double_tap::DoubleTapResetRp,
    keyscan::duplex_matrix::create_duplex_matrix,
    mouse::pmw3360::create_pmw3360,
    panic_utils,
    split::pio_half_duplex::PioHalfDuplexSplitDriver,
    usb::{new_usb, UsbOpts},
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
    PIO1_IRQ_0 => embassy_rp::pio::InterruptHandler<PIO1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut cfg = embassy_rp::config::Config::default();
    cfg.clocks.sys_clk.div_int = 2;
    let p = embassy_rp::init(cfg);

    let dtr = DoubleTapResetRp;

    let display = create_ssd1306(
        p.I2C1,
        Irqs,
        p.PIN_2,
        p.PIN_3,
        ssd1306::size::DisplaySize128x32,
    );

    let Some(display) = panic_utils::display_message_if_panicked(display).await else {
        cortex_m::asm::udf()
    };

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
        let mut config = rktk_drivers_rp2040::usb::Config::new(0xc0de, 0xcafe);
        config.manufacturer = Some("Yowkees/nazo6");
        config.product = Some("keyball");
        config.serial_number = Some("12345678");
        config.max_power = 100;
        config.max_packet_size_0 = 64;
        config.supports_remote_wakeup = true;
        let driver = embassy_rp::usb::Driver::new(p.USB, Irqs);
        let usb_opts = UsbOpts {
            config,
            mouse_poll_interval: 5,
            kb_poll_interval: 5,
            driver,
        };

        new_usb(usb_opts)
    };

    let pio = Pio::new(p.PIO0, Irqs);
    let split = PioHalfDuplexSplitDriver::new(pio, p.PIN_1);

    let pio = Pio::new(p.PIO1, Irqs);
    let backlight = Ws2812Pio::new(pio, p.PIN_0, p.DMA_CH2);

    let storage;
    rktk_drivers_rp2040::init_storage!(storage, p.FLASH, p.DMA_CH3, { 4 * 1024 * 1024 });

    let drivers = Drivers {
        key_scanner,
        double_tap_reset: Some(dtr),
        mouse_builder: Some(ball),
        usb_builder: Some(usb),
        display_builder: Some(display),
        split,
        backlight: Some(backlight),
        ble: Option::<DummyBleDriver>::None,
        storage: Some(storage),
    };

    rktk::task::start(drivers, keymap::KEY_CONFIG).await;
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    panic_utils::save_panic_info(info);
    cortex_m::peripheral::SCB::sys_reset()
}
