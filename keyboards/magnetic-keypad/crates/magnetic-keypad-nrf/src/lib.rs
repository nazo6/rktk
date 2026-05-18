#![no_std]

pub mod keymap;
pub mod xw09d;

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
        profile::{LinearProfile, SingleProfileMap},
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
    TWISPI0 => embassy_nrf::twim::InterruptHandler<embassy_nrf::peripherals::TWISPI0>;
});

pub async fn run(spawner: Spawner, keymap: &'static Keymap) {
    let p = embassy_nrf::init(Default::default());

    // Spawn XW09D touch controller background task (polling ch0-3 for keys, ch4-8 for slider)
    spawner.spawn(touch_task(p.TWISPI0, p.P0_06.into(), p.P0_08.into()).unwrap());
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
    let storage =
        rktk_drivers_common::storage::flash_sequential_map::FlashSequentialMapStorage::new(
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

#[embassy_executor::task]
pub async fn touch_task(
    twispi0: embassy_nrf::Peri<'static, embassy_nrf::peripherals::TWISPI0>,
    sda: embassy_nrf::Peri<'static, embassy_nrf::gpio::AnyPin>,
    scl: embassy_nrf::Peri<'static, embassy_nrf::gpio::AnyPin>,
) {
    let mut config = embassy_nrf::twim::Config::default();
    config.frequency = embassy_nrf::twim::Frequency::K400;
    let mut i2c_buf = [0u8; 64];
    let twim = embassy_nrf::twim::Twim::new(twispi0, Irqs, sda, scl, config, &mut i2c_buf);

    let mut touch_sensor = xw09d::Xw09d::new(twim);
    let kb_sender = rktk::hooks::channels::report::keyboard_event_sender();
    let enc_sender = rktk::hooks::channels::report::encoder_event_sender();

    let mut prev_buttons = [false; 4];
    let mut prev_slider_pos: Option<i32> = None;
    const SLIDER_THRESHOLD: i32 = 25; // 0.25 of one pad spacing

    loop {
        if let Ok(state) = touch_sensor.read_touch().await {
            // 1. Right-side Buttons (ch0 - ch3)
            for i in 0..4 {
                let current = state.pads[i];
                if current != prev_buttons[i] {
                    let ev = rktk::drivers::interface::keyscan::KeyChangeEvent {
                        row: i as u8,
                        col: 3, // Column 3 is the virtual column
                        pressed: current,
                    };
                    let _ = kb_sender.try_send(ev);
                    prev_buttons[i] = current;
                }
            }

            // 2. Left-side Slider (ch4 - ch8)
            let mut sum = 0i32;
            let mut count = 0i32;
            for i in 4..=8 {
                if state.pads[i] {
                    sum += (i as i32 - 4) * 100; // Map pads 4..8 to 0..400
                    count += 1;
                }
            }

            if count > 0 {
                let pos = sum / count;
                if let Some(prev_pos) = prev_slider_pos {
                    let diff = pos - prev_pos;
                    if diff >= SLIDER_THRESHOLD {
                        let _ = enc_sender.try_send((
                            1,
                            rktk::drivers::interface::encoder::EncoderDirection::Clockwise,
                        ));
                        prev_slider_pos = Some(pos);
                    } else if diff <= -SLIDER_THRESHOLD {
                        let _ = enc_sender.try_send((
                            1,
                            rktk::drivers::interface::encoder::EncoderDirection::CounterClockwise,
                        ));
                        prev_slider_pos = Some(pos);
                    }
                } else {
                    prev_slider_pos = Some(pos);
                }
            } else {
                prev_slider_pos = None;
            }
        }
        embassy_time::Timer::after_millis(15).await;
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    rktk_drivers_common::panic_utils::save_panic_info(info);
    cortex_m::peripheral::SCB::sys_reset()
}
