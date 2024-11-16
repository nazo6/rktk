#![no_std]
#![no_main]

use core::panic::PanicInfo;

use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    gpio::Flex,
    peripherals::{I2C1, PIO0, PIO1, USB},
    pio::Pio,
};

use embassy_sync::{blocking_mutex::raw::NoopRawMutex, mutex::Mutex};
use rktk::{
    hooks::create_empty_hooks,
    interface::{
        ble::DummyBleDriverBuilder, debounce::NoopDebounceDriver, encoder::DummyEncoderDriver,
    },
    task::Drivers,
};
use rktk_drivers_rp::{
    backlight::ws2812_pio::Ws2812Pio,
    display::ssd1306::create_ssd1306,
    double_tap::DoubleTapResetRp,
    keyscan::duplex_matrix::create_duplex_matrix,
    mouse::paw3395,
    panic_utils,
    split::pio_half_duplex::PioHalfDuplexSplitDriver,
    usb::{new_usb, UsbOpts},
};

use keyball_common::*;

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

    let ball_spi = Mutex::<NoopRawMutex, _>::new(embassy_rp::spi::Spi::new(
        p.SPI0,
        p.PIN_22,
        p.PIN_23,
        p.PIN_20,
        p.DMA_CH0,
        p.DMA_CH1,
        paw3395::recommended_spi_config(),
    ));
    let ball = paw3395::create_paw3395(&ball_spi, p.PIN_21, PAW3395_CONFIG);

    let keyscan = create_duplex_matrix::<'_, 5, 4, 5, 7>(
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
        translate_key_position,
    );

    let usb = {
        let driver = embassy_rp::usb::Driver::new(p.USB, Irqs);
        let usb_opts = UsbOpts {
            config: USB_CONFIG,
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

    // NOTE: needed for some macro thing. maybe this can be avoided.
    #[allow(clippy::needless_late_init)]
    let storage;
    rktk_drivers_rp::init_storage!(storage, p.FLASH, p.DMA_CH3, { 4 * 1024 * 1024 });

    let drivers = Drivers {
        keyscan,
        double_tap_reset: Some(dtr),
        mouse_builder: Some(ball),
        usb_builder: Some(usb),
        display_builder: Some(display),
        split,
        backlight: Some(backlight),
        ble_builder: Option::<DummyBleDriverBuilder>::None,
        storage: Some(storage),
        debounce: NoopDebounceDriver,
        encoder: Option::<DummyEncoderDriver>::None,
    };

    rktk::task::start(drivers, keymap::KEY_CONFIG, create_empty_hooks()).await;
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    panic_utils::save_panic_info(info);
    cortex_m::peripheral::SCB::sys_reset()
}
