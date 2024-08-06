#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use core::panic::PanicInfo;

use embassy_executor::Spawner;
use embassy_nrf::{
    gpio::{Flex, Pin},
    interrupt::{self, InterruptExt, Priority},
    peripherals::SPI2,
    ppi::Group,
    usb::vbus_detect::SoftwareVbusDetect,
};
use once_cell::sync::OnceCell;

use rktk::{
    interface::{double_tap::DummyDoubleTapResetDriver, rand::RandomDriver as _},
    task::Drivers,
};
use rktk_drivers_nrf52::{
    backlight::ws2812_pwm::Ws2812Pwm, display::ssd1306::create_ssd1306,
    keyscan::duplex_matrix::create_duplex_matrix, mouse::pmw3360::create_pmw3360,
    softdevice::ble::init_ble_server, split::uart_half_duplex::UartHalfDuplexSplitDriver,
};

use defmt_rtt as _;
use nrf_softdevice as _;

#[cfg(not(feature = "ble-master"))]
use rktk::interface::ble::DummyBleDriver;
#[cfg(not(feature = "usb"))]
use rktk::interface::usb::DummyUsbDriver;
#[cfg(feature = "ble-master")]
use rktk_drivers_nrf52::softdevice::ble::NrfBleDriver;
#[cfg(feature = "usb")]
use rktk_drivers_nrf52::usb::{new_usb, UsbConfig, UsbUserOpts};

mod keymap;

use embassy_nrf::{bind_interrupts, peripherals::USBD};

bind_interrupts!(pub struct Irqs {
    USBD => embassy_nrf::usb::InterruptHandler<USBD>;
    SPIM2_SPIS2_SPI2 => embassy_nrf::spim::InterruptHandler<SPI2>;
    SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => embassy_nrf::twim::InterruptHandler<embassy_nrf::peripherals::TWISPI0>;
    UARTE0_UART0 => embassy_nrf::buffered_uarte::InterruptHandler<embassy_nrf::peripherals::UARTE0>;
});

static SOFTWARE_VBUS: OnceCell<SoftwareVbusDetect> = OnceCell::new();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // - About limitation of softdevice
    // By enabling softdevice, some interrupt priority level (P0,P1,P4)
    // and peripherals are reserved by softdevice, and using them causes panic.
    //
    // Example reserved peripherals are:
    // - TIMER0
    // - CLOCK
    // - RTC0
    // ... and more
    //
    // ref:
    // List of reserved peripherals: https://docs.nordicsemi.com/bundle/sds_s140/page/SDS/s1xx/sd_resource_reqs/hw_block_interrupt_vector.html
    // Peripheral register addresses: https://docs.nordicsemi.com/bundle/ps_nrf52840/page/memory.html
    //
    // When panic occurs by peripheral conflict, PC address that caused panic is logged.
    // By investigating the address using decompiler tools like ghidra, you can find the peripheral that caused the panic.

    let mut config = embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = Priority::P2;
    config.time_interrupt_priority = Priority::P2;
    let p = embassy_nrf::init(config);

    interrupt::USBD.set_priority(Priority::P2);
    interrupt::SPIM2_SPIS2_SPI2.set_priority(Priority::P2);
    interrupt::SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0.set_priority(Priority::P2);
    interrupt::UARTE1.set_priority(Priority::P2);

    let ball = create_pmw3360(p.SPI2, Irqs, p.P1_13, p.P1_11, p.P0_10, p.P0_09);

    let key_scanner = create_duplex_matrix::<'_, 5, 4, 5, 7>(
        [
            Flex::new(p.P0_22),
            Flex::new(p.P0_24),
            Flex::new(p.P1_00),
            Flex::new(p.P0_11),
            Flex::new(p.P1_04),
        ],
        [
            Flex::new(p.P0_31),
            Flex::new(p.P0_29),
            Flex::new(p.P0_02),
            Flex::new(p.P1_15),
        ],
        (2, 6),
    );

    let display = create_ssd1306(
        p.TWISPI0,
        Irqs,
        p.P0_17,
        p.P0_20,
        ssd1306::size::DisplaySize128x32,
    );

    let split = UartHalfDuplexSplitDriver::new(
        p.P0_08.degrade(),
        p.UARTE0,
        Irqs,
        p.TIMER1,
        p.PPI_CH0,
        p.PPI_CH1,
        p.PPI_GROUP0.degrade(),
    );

    let backlight = Ws2812Pwm::new(p.PWM0, p.P0_06);

    let sd = rktk_drivers_nrf52::softdevice::init_sd("keyball61");
    let (server, sd) = init_ble_server(sd).await;
    rktk_drivers_nrf52::softdevice::start_softdevice(sd).await;

    embassy_time::Timer::after_millis(50).await;

    let rand = rktk_drivers_nrf52::softdevice::rand::SdRand::new(sd);

    let storage =
        rktk_drivers_nrf52::softdevice::flash::init_database(sd, rand.get_random().unwrap());

    let ble = {
        #[cfg(feature = "ble-master")]
        let ble = Some(NrfBleDriver::new(sd, server, "keyball61", storage).await);

        #[cfg(not(feature = "ble-master"))]
        let ble = Option::<DummyBleDriver>::None;

        ble
    };

    let drivers = Drivers {
        key_scanner,
        double_tap_reset: Option::<DummyDoubleTapResetDriver>::None,
        mouse_builder: Some(ball),
        usb: {
            #[cfg(feature = "usb")]
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
                Some(new_usb(usb_opts, driver).await)
            };

            #[cfg(not(feature = "usb"))]
            let usb = Option::<DummyUsbDriver>::None;

            usb
        },
        display_builder: Some(display),
        split: Some(split),
        backlight: Some(backlight),
        storage: Some(storage),
        ble,
    };

    rktk::task::start(drivers, keymap::KEYMAP).await;
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use embassy_nrf::peripherals::{P0_17, P0_20, TWISPI0};

    let display = unsafe {
        create_ssd1306(
            TWISPI0::steal(),
            Irqs,
            P0_17::steal(),
            P0_20::steal(),
            ssd1306::size::DisplaySize128x32,
        )
    }
    .build_sync()
    .unwrap();

    rktk::panicking::display_panic_message(display, info);

    cortex_m::asm::udf()
}
