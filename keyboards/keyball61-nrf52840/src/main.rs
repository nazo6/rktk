#![no_std]
#![no_main]

use core::panic::PanicInfo;

use defmt_rtt as _;

use display::GlobalDisplay;
use embassy_executor::Spawner;
use nrf_softdevice as _;

use embassy_nrf::{gpio::Flex, peripherals::SPI2, usb::vbus_detect::SoftwareVbusDetect};
use oled::Oled;
use once_cell::sync::OnceCell;
use rktk::interface::{double_tap::DummyDoubleTapReset, mouse::DummyMouse};
use rktk_drivers_nrf52::{
    keyscan::duplex_matrix::create_duplex_matrix,
    mouse::pmw3360::create_pmw3360,
    usb::{UsbConfig, UsbDriver, UsbUserOpts},
};

mod display;
mod keymap;
mod oled;

use embassy_nrf::{bind_interrupts, peripherals::USBD};

bind_interrupts!(pub struct Irqs {
    USBD => embassy_nrf::usb::InterruptHandler<USBD>;
    SPIM2_SPIS2_SPI2 => embassy_nrf::spim::InterruptHandler<SPI2>;
    SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => embassy_nrf::twim::InterruptHandler<embassy_nrf::peripherals::TWISPI0>;
});

static SOFTWARE_VBUS: OnceCell<SoftwareVbusDetect> = OnceCell::new();
pub static DISPLAY: GlobalDisplay = GlobalDisplay::new();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    let oled = Oled::new(p.TWISPI0, Irqs, p.P0_17, p.P0_20);
    DISPLAY.init(oled.0).await;

    DISPLAY.set_message("hello").await;

    // let Ok(ball) = create_pmw3360(p.SPI2, Irqs, p.P1_13, p.P1_11, p.P0_10, p.P0_09).await else {
    //     panic!("Failed to create PMW3360");
    // };

    let rows = [
        Flex::new(p.P0_22),
        Flex::new(p.P0_24),
        Flex::new(p.P1_00),
        Flex::new(p.P0_11),
        Flex::new(p.P1_04),
    ];
    let cols = [
        Flex::new(p.P1_15),
        Flex::new(p.P0_02),
        Flex::new(p.P0_29),
        Flex::new(p.P0_31),
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

    let vbus = SOFTWARE_VBUS.get_or_init(|| SoftwareVbusDetect::new(true, true));

    let driver = embassy_nrf::usb::Driver::new(p.USBD, Irqs, vbus);

    let usb = UsbDriver::create_and_start(usb_opts, driver).await;

    DISPLAY.set_message("USB OK").await;

    rktk::task::start(
        Option::<DummyDoubleTapReset>::None,
        key_scanner,
        Option::<DummyMouse>::None,
        usb,
        keymap::KEYMAP,
    )
    .await;

    DISPLAY.set_message("exit").await;
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    DISPLAY.try_set_message("panic");
    loop {}
}
