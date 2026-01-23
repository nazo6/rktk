#![no_std]

use core::panic::PanicInfo;

use embassy_nrf::{
    bind_interrupts,
    gpio::{Input, Level, Output, OutputDrive, Pull},
    interrupt::{self, InterruptExt, Priority},
    spim::Spim,
    twim::Twim,
};
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, mutex::Mutex};
use keyball61_common::*;

use rktk::{
    config::keymap::Keymap,
    drivers::{Drivers, dummy},
    hooks::create_empty_hooks,
};
use rktk_drivers_common::{
    debounce::EagerDebounceDriver,
    display::ssd1306::{self, Ssd1306Driver, prelude::DisplaySize128x32},
    keyscan::{detect_hand_from_matrix, duplex_matrix::DuplexMatrixScanner},
    mouse::pmw3360::Pmw3360,
    panic_utils,
    spi::EmbassySpiDevice,
};
use rktk_drivers_nrf::{
    keyscan::flex_pin::NrfFlexPin, rgb::ws2812_pwm::Ws2812Pwm,
    split::uart_half_duplex::UartHalfDuplexSplitDriver, system::NrfSystemDriver,
};

use nrf_softdevice as _;

#[cfg(feature = "usb")]
mod usb {
    pub use rktk_drivers_common::usb::*;
}

bind_interrupts!(pub struct Irqs {
    USBD => embassy_nrf::usb::InterruptHandler<embassy_nrf::peripherals::USBD>;
    SPI2 => embassy_nrf::spim::InterruptHandler<embassy_nrf::peripherals::SPI2>;
    TWISPI0 => embassy_nrf::twim::InterruptHandler<embassy_nrf::peripherals::TWISPI0>;
    UARTE0 => embassy_nrf::buffered_uarte::InterruptHandler<embassy_nrf::peripherals::UARTE0>;
});

pub async fn start(spawner: embassy_executor::Spawner, keymap: &'static Keymap) {
    let mut config = embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = Priority::P2;
    config.time_interrupt_priority = Priority::P2;
    let mut p = embassy_nrf::init(config);

    interrupt::USBD.set_priority(Priority::P2);
    interrupt::SPI2.set_priority(Priority::P2);
    interrupt::TWISPI0.set_priority(Priority::P2);
    interrupt::UARTE0.set_priority(Priority::P2);

    let mut display = Ssd1306Driver::new(
        Twim::new(
            p.TWISPI0,
            Irqs,
            p.P0_17,
            p.P0_20,
            rktk_drivers_nrf::display::ssd1306::recommended_i2c_config(),
            &mut [],
        ),
        DisplaySize128x32,
        ssd1306::prelude::DisplayRotation::Rotate90,
    );

    panic_utils::display_message_if_panicked(&mut display).await;

    let bus = Mutex::<NoopRawMutex, _>::new(Spim::new(
        p.SPI2,
        Irqs,
        p.P1_13,
        p.P1_11,
        p.P0_10,
        rktk_drivers_nrf::mouse::pmw3360::recommended_spi_config(),
    ));
    let spi = EmbassySpiDevice::new(
        &bus,
        Output::new(
            p.P0_06,
            embassy_nrf::gpio::Level::High,
            embassy_nrf::gpio::OutputDrive::Standard,
        ),
    );
    let ball = Pmw3360::new(spi, Default::default());

    let hand = detect_hand_from_matrix(
        Output::new(p.P1_00.reborrow(), Level::Low, OutputDrive::Standard),
        Input::new(p.P1_15.reborrow(), Pull::Down),
        None,
        None,
    )
    .await
    .unwrap();
    let keyscan = DuplexMatrixScanner::<_, _, 5, 4, 5, 7>::new(
        [
            NrfFlexPin::new(p.P0_22), // ROW0
            NrfFlexPin::new(p.P0_24), // ROW1
            NrfFlexPin::new(p.P1_00), // ROW2
            NrfFlexPin::new(p.P0_11), // ROW3
            NrfFlexPin::new(p.P1_04), // ROW4
        ],
        [
            NrfFlexPin::new(p.P0_31), // COL0
            NrfFlexPin::new(p.P0_29), // COL1
            NrfFlexPin::new(p.P0_02), // COL2
            NrfFlexPin::new(p.P1_15), // COL3
        ],
        None,
        translate_key_position(hand),
    );

    let split = UartHalfDuplexSplitDriver::new(
        p.P0_08,
        p.UARTE0,
        Irqs,
        p.TIMER1,
        p.PPI_CH0,
        p.PPI_CH1,
        p.PPI_GROUP0,
    );

    let rgb = Ws2812Pwm::<1024, _, _>::new(p.PWM0, p.P0_09);

    #[cfg(feature = "ble")]
    let sd = rktk_drivers_nrf::softdevice::init_softdevice("keyball61");
    #[cfg(feature = "ble")]
    let server = {
        let server = rktk_drivers_nrf::softdevice::ble::init_ble_server(
            sd,
            rktk_drivers_nrf::softdevice::ble::DeviceInformation {
                manufacturer_name: Some("nazo6"),
                model_number: Some("100"),
                serial_number: Some("100"),
                ..Default::default()
            },
        );
        rktk_drivers_nrf::softdevice::start_softdevice(spawner, sd);
        embassy_time::Timer::after_millis(50).await;
        server
    };
    #[cfg(feature = "ble")]
    let (storage, ble_builder) = {
        use rktk_drivers_common::storage::flash_sequential_map::FlashSequentialMapStorage;
        pub use rktk_drivers_nrf::softdevice::ble::SoftdeviceBleReporterBuilder;

        let (part_main, part_bond) =
            rktk_drivers_nrf::softdevice::flash::get_typical_flash_partitions(sd);
        let part_main_size = part_main.size();
        (
            Some(FlashSequentialMapStorage::new(part_main, 0, part_main_size)),
            Some(SoftdeviceBleReporterBuilder::new(
                spawner,
                sd,
                server,
                "keyball61",
                part_bond,
            )),
        )
    };
    #[cfg(not(feature = "ble"))]
    let (ble_builder, storage) = (dummy::ble_builder(), dummy::storage());

    let drivers = Drivers {
        keyscan,
        system: NrfSystemDriver::new(None),
        mouse: Some(ball),
        usb_builder: {
            #[cfg(feature = "usb")]
            let usb = {
                let embassy_driver = embassy_nrf::usb::Driver::new(
                    p.USBD,
                    Irqs,
                    rktk_drivers_nrf::get_vbus!(spawner, Irqs),
                );
                let mut driver_config = usb::UsbDriverConfig::new(0xc0de, 0xcafe);
                driver_config.product = Some("Keyball61");
                let opts = usb::CommonUsbDriverConfig::new(embassy_driver, driver_config);
                Some(usb::CommonUsbReporterBuilder::new(opts))
            };

            #[cfg(not(feature = "usb"))]
            let usb = dummy::usb_builder();

            usb
        },
        display: Some(display),
        split: Some(split),
        rgb: Some(rgb),
        storage,
        ble_builder,
        debounce: Some(EagerDebounceDriver::new(
            embassy_time::Duration::from_millis(20),
            true,
        )),
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
