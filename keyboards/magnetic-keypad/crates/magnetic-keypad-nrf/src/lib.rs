#![no_std]

pub mod keymap;

use core::panic::PanicInfo;

use embassy_executor::Spawner;
use embassy_nrf::{
    bind_interrupts,
    gpio::{Level, Output, OutputDrive},
    saadc,
};
use rktk::{
    config::keymap::Keymap,
    config::new_rktk_opts,
    drivers::{Drivers, dummy},
    hooks::empty_hooks::create_empty_hooks,
};

use rktk_drivers_common::{
    magnetic::{
        matrix::{MagneticMatrix, MuxScanner},
        mux::sn74lv4051::Sn74lv4051,
        profile::{SingleProfileMap, LinearProfile},
    },
    usb::{CommonUsbDriverConfig, CommonUsbReporterBuilder, UsbDriverConfig},
};
use rktk_drivers_nrf::{
    keyscan::magnetic::NrfAdc, rgb::ws2812_pwm::Ws2812Pwm, system::NrfSystemDriver,
};

bind_interrupts!(struct Irqs {
    USBD => embassy_nrf::usb::InterruptHandler<embassy_nrf::peripherals::USBD>;
    SAADC => saadc::InterruptHandler;
    CLOCK_POWER => embassy_nrf::usb::vbus_detect::InterruptHandler;
});

pub async fn run(spawner: Spawner, keymap: &'static Keymap) {
    let p = embassy_nrf::init(Default::default());
    // Multiplexer selection pins
    // SEL A: P0.29, SEL B: P0.02, SEL C: P1.15
    let mux_s0 = Output::new(p.P0_29, Level::Low, OutputDrive::Standard);
    let mux_s1 = Output::new(p.P0_02, Level::Low, OutputDrive::Standard);
    let mux_s2 = Output::new(p.P1_15, Level::Low, OutputDrive::Standard);

    let mux = Sn74lv4051::new(mux_s0, mux_s1, mux_s2);

    // ADC for multiplexer output: P0.31 (AIN7)
    let config = saadc::Config::default();
    let mut channel_config = saadc::ChannelConfig::single_ended(p.P0_31);
    channel_config.time = saadc::Time::_40US;
    let saadc = saadc::Saadc::new(p.SAADC, Irqs, config, [channel_config]);
    let adc = NrfAdc::new(saadc);

    let scanner = MuxScanner::new(adc, mux, |row, col| {
        match (row, col) {
            (2, 0) => Some((0, 0)), // KEY1
            (2, 1) => Some((1, 0)), // KEY2
            (2, 2) => Some((2, 0)), // KEY3
            (1, 0) => Some((3, 0)), // KEY4
            (1, 1) => Some((4, 0)), // KEY5
            (1, 2) => Some((5, 0)), // KEY6
            (0, 0) => Some((6, 0)), // KEY7
            (0, 1) => Some((7, 0)), // KEY8
            _ => None,
        }
    });

    let profile = LinearProfile { max_travel: 400 };
    let profile_map = SingleProfileMap { profile };

    let keyscan = MagneticMatrix::<
        _,
        _,
        { rktk::config::CONST_CONFIG.keyboard.rows as usize },
        { rktk::config::CONST_CONFIG.keyboard.cols as usize },
    >::new(scanner, profile_map, 30, 20, 15);

    // RGB: P0.11, 8 LEDs
    let rgb = Ws2812Pwm::<256, _, _>::new(p.PWM0, p.P0_11);

    // Encoder: P0.09(A), P0.10(B)
    let encoder = rktk_drivers_common::encoder::GeneralEncoder::new([(
        embassy_nrf::gpio::Input::new(p.P0_09, embassy_nrf::gpio::Pull::Up),
        embassy_nrf::gpio::Input::new(p.P0_10, embassy_nrf::gpio::Pull::Up),
    )]);

    let flash = embassy_nrf::nvmc::Nvmc::new(p.NVMC);
    let async_flash = embassy_embedded_hal::adapter::BlockingAsync::new(flash);
    let storage = rktk_drivers_common::storage::flash_sequential_map::FlashSequentialMapStorage::new(
        async_flash,
        0xFC000,
        16 * 1024,
    );

    let drivers = Drivers {
        keyscan,
        system: NrfSystemDriver::new(None),
        mouse: dummy::mouse(),
        usb_builder: Some({
            let embassy_driver = embassy_nrf::usb::Driver::new(
                p.USBD,
                Irqs,
                rktk_drivers_nrf::get_vbus!(spawner, Irqs),
            );
            let mut driver_config = UsbDriverConfig::new(0xc0de, 0xcaee);
            driver_config.product = Some("kp");
            let opts = CommonUsbDriverConfig::new(embassy_driver, driver_config);

            CommonUsbReporterBuilder::new(opts)
        }),
        display: dummy::display(),
        split: dummy::split(),
        rgb: Some(rgb),
        ble_builder: dummy::ble_builder(),
        storage: Some(storage),
        debounce: dummy::debounce(), // Magnetic matrix handles its own "debounce" via RT logic
        encoder: Some(encoder),
    };

    rktk::task::start(
        spawner,
        drivers,
        create_empty_hooks(),
        new_rktk_opts(keymap, None),
    )
    .await;
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    rktk_drivers_common::panic_utils::save_panic_info(info);
    cortex_m::peripheral::SCB::sys_reset()
}
