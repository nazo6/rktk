#![no_std]

use core::panic::PanicInfo;

use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_rp::{
    bind_interrupts,
    gpio::{Input, Level, Output},
    i2c::I2c,
    peripherals::{I2C1, PIO0, PIO1, USB},
    pio::Pio,
};

use embassy_sync::{blocking_mutex::raw::NoopRawMutex, mutex::Mutex};
use rktk::{
    config::keymap::Keymap,
    drivers::{Drivers, dummy},
    hooks::create_empty_hooks,
};
use rktk_drivers_common::{
    display::ssd1306::{self, Ssd1306Driver},
    keyscan::{detect_hand_from_matrix, duplex_matrix::DuplexMatrixScanner},
    mouse::pmw3360::Pmw3360,
    panic_utils,
    usb::*,
};
use rktk_drivers_rp::{
    keyscan::flex_pin::RpFlexPin, mouse::pmw3360, rgb::ws2812_pio::Ws2812Pio,
    split::pio_half_duplex::PioHalfDuplexSplitDriver,
};

use keyball61_common::*;

bind_interrupts!(pub struct Irqs {
    USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
    I2C1_IRQ => embassy_rp::i2c::InterruptHandler<I2C1>;
    PIO0_IRQ_0 => embassy_rp::pio::InterruptHandler<PIO0>;
    PIO1_IRQ_0 => embassy_rp::pio::InterruptHandler<PIO1>;
});

pub async fn start(spawner: embassy_executor::Spawner, keymap: &'static Keymap) {
    let mut cfg = embassy_rp::config::Config::default();
    cfg.clocks.sys_clk.div_int = 2;
    let mut p = embassy_rp::init(cfg);

    let mut display = Ssd1306Driver::new(
        I2c::new_async(
            p.I2C1,
            p.PIN_3,
            p.PIN_2,
            Irqs,
            rktk_drivers_rp::display::ssd1306::recommended_i2c_config(),
        ),
        ssd1306::prelude::DisplaySize128x32,
        ssd1306::prelude::DisplayRotation::Rotate90,
    );

    panic_utils::display_message_if_panicked(&mut display).await;

    let spi = Mutex::<NoopRawMutex, _>::new(embassy_rp::spi::Spi::new(
        p.SPI0,
        p.PIN_22,
        p.PIN_23,
        p.PIN_20,
        p.DMA_CH0,
        p.DMA_CH1,
        pmw3360::recommended_spi_config(),
    ));
    let ball_spi = SpiDevice::new(&spi, Output::new(p.PIN_21, embassy_rp::gpio::Level::High));
    let ball = Pmw3360::new(ball_spi);

    let hand = detect_hand_from_matrix(
        Output::new(p.PIN_6.reborrow(), Level::Low),
        Input::new(p.PIN_26.reborrow(), embassy_rp::gpio::Pull::Down),
        None,
        None,
    )
    .await
    .unwrap();
    let keyscan = DuplexMatrixScanner::<_, _, 5, 4, 5, 7>::new(
        [
            RpFlexPin::new(p.PIN_4),
            RpFlexPin::new(p.PIN_5),
            RpFlexPin::new(p.PIN_6),
            RpFlexPin::new(p.PIN_7),
            RpFlexPin::new(p.PIN_8),
        ],
        [
            RpFlexPin::new(p.PIN_29),
            RpFlexPin::new(p.PIN_28),
            RpFlexPin::new(p.PIN_27),
            RpFlexPin::new(p.PIN_26),
        ],
        Some(rktk_drivers_common::keyscan::duplex_matrix::OutputWait::Pin),
        translate_key_position(hand),
    );

    let usb = {
        let embassy_driver = embassy_rp::usb::Driver::new(p.USB, Irqs);
        let mut driver_config = UsbDriverConfig::new(0xc0de, 0xcafe);
        driver_config.product = Some("Keyball61");
        let opts = CommonUsbDriverConfig::new(embassy_driver, driver_config);
        Some(CommonUsbReporterBuilder::new(opts))
    };

    let pio = Pio::new(p.PIO0, Irqs);
    let split = PioHalfDuplexSplitDriver::new(pio, p.PIN_1);

    let pio = Pio::new(p.PIO1, Irqs);
    let rgb = Ws2812Pio::<'_, 30, _>::new(pio, p.PIN_0, p.DMA_CH2);

    rktk_drivers_rp::init_storage!(storage, p.FLASH, p.DMA_CH3, { 4 * 1024 * 1024 });

    let drivers = Drivers {
        keyscan,
        system: rktk_drivers_rp::system::RpSystemDriver,
        mouse: Some(ball),
        usb_builder: usb,
        display: Some(display),
        split: Some(split),
        rgb: Some(rgb),
        ble_builder: dummy::ble_builder(),
        storage: Some(storage),
        debounce: dummy::debounce(),
        encoder: dummy::encoder(),
    };

    match hand {
        rktk::config::Hand::Left => {
            rktk::task::start(
                spawner,
                drivers,
                create_empty_hooks(),
                get_opts_left(keymap),
            )
            .await;
        }
        rktk::config::Hand::Right => {
            rktk::task::start(
                spawner,
                drivers,
                create_empty_hooks(),
                get_opts_right(keymap),
            )
            .await;
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    panic_utils::save_panic_info(info);
    cortex_m::peripheral::SCB::sys_reset()
}
