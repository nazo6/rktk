#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use core::{
    ops::{Deref, DerefMut as _},
    panic::PanicInfo,
};

use defmt_rtt as _;

use embassy_executor::Spawner;
use nrf_softdevice as _;

use embassy_nrf::{gpio::Flex, peripherals::SPI2, usb::vbus_detect::SoftwareVbusDetect};
use once_cell::sync::OnceCell;
use rktk::{
    interface::{
        backlight::DummyBacklightDriver, double_tap::DummyDoubleTapResetDriver,
        mouse::DummyMouseDriver, split::DummySplitDriver,
    },
    task::Drivers,
};
use rktk_drivers_nrf52::{
    display::ssd1306::{create_ssd1306, Ssd1306Display},
    keyscan::duplex_matrix::create_duplex_matrix,
    mouse::pmw3360::create_pmw3360,
    usb::{new_usb, UsbConfig, UsbUserOpts},
};

mod keymap;

use embassy_nrf::{bind_interrupts, peripherals::USBD};

bind_interrupts!(pub struct Irqs {
    USBD => embassy_nrf::usb::InterruptHandler<USBD>;
    SPIM2_SPIS2_SPI2 => embassy_nrf::spim::InterruptHandler<SPI2>;
    SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => embassy_nrf::twim::InterruptHandler<embassy_nrf::peripherals::TWISPI0>;
});

static SOFTWARE_VBUS: OnceCell<SoftwareVbusDetect> = OnceCell::new();

static BUILD_TIME: &str = build_time::build_time_local!("              %H%M%S");

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    // let ball = create_pmw3360(p.SPI2, Irqs, p.P1_13, p.P1_11, p.P0_10, p.P0_09).await;

    let key_scanner = create_duplex_matrix::<'_, 5, 4, 5, 7>(
        [
            Flex::new(p.P0_22),
            Flex::new(p.P0_24),
            Flex::new(p.P1_00),
            Flex::new(p.P0_11),
            Flex::new(p.P1_04),
        ],
        [
            Flex::new(p.P1_15),
            Flex::new(p.P0_02),
            Flex::new(p.P0_29),
            Flex::new(p.P0_31),
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

        let vbus = SOFTWARE_VBUS.get_or_init(|| SoftwareVbusDetect::new(true, true));
        let driver = embassy_nrf::usb::Driver::new(p.USBD, Irqs, vbus);
        new_usb(usb_opts, driver).await
    };

    let display = create_ssd1306(
        p.TWISPI0,
        Irqs,
        p.P0_17,
        p.P0_20,
        ssd1306::size::DisplaySize128x32,
    );

    let drivers = Drivers {
        key_scanner,
        double_tap_reset: Option::<DummyDoubleTapResetDriver>::None,
        mouse: Option::<DummyMouseDriver>::None,
        usb,
        display: Some(display),
        split: Option::<DummySplitDriver>::None,
        backlight: Option::<DummyBacklightDriver>::None,
    };

    rktk::print_str!(BUILD_TIME);

    rktk::task::start(drivers, keymap::KEYMAP).await;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    use embassy_nrf::peripherals::{P0_17, P0_20, TWISPI0};
    use rktk::interface::display::DisplayDriver as _;
    use ssd1306::mode::DisplayConfig as _;

    let mut display = unsafe {
        create_ssd1306(
            TWISPI0::steal(),
            Irqs,
            P0_17::steal(),
            P0_20::steal(),
            ssd1306::size::DisplaySize128x32,
        )
    };
    let _ = display.deref_mut().init();
    let _ = display.update_text_sync("panic", embedded_graphics::prelude::Point { x: 0, y: 0 });
    loop {}
}
