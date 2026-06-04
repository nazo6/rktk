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
    drivers::{Drivers, dummy},
    hooks::empty_hooks::create_empty_hooks,
};

use rktk_drivers_common::{
    display::mipi_display::MipiDisplayDriver,
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
    TWISPI1 => embassy_nrf::spim::InterruptHandler<embassy_nrf::peripherals::TWISPI1>;
});

mod display {
    use display_driver_st7789::{St7789Spec, impl_st7789_generic, spec::PanelSpec};

    pub struct MyCustomPanel;

    impl PanelSpec for MyCustomPanel {
        const PHYSICAL_WIDTH: u16 = 76;
        const PHYSICAL_HEIGHT: u16 = 284;

        const PHYSICAL_X_OFFSET: u16 = 82;
        const PHYSICAL_Y_OFFSET: u16 = 18;

        const INVERTED: bool = false;
        const BGR: bool = false;
    }

    impl_st7789_generic!(MyCustomPanel);
}

type DisplayType = MipiDisplayDriver<
    display_driver_spi::SpiDisplayBus<
        embedded_hal_bus::spi::ExclusiveDevice<
            embassy_nrf::spim::Spim<'static>,
            embassy_nrf::gpio::Output<'static>,
            embassy_time::Delay,
        >,
        embassy_nrf::gpio::Output<'static>,
    >,
    display_driver_st7789::St7789<
        display::MyCustomPanel,
        embassy_nrf::gpio::Output<'static>,
        display_driver_spi::SpiDisplayBus<
            embedded_hal_bus::spi::ExclusiveDevice<
                embassy_nrf::spim::Spim<'static>,
                embassy_nrf::gpio::Output<'static>,
                embassy_time::Delay,
            >,
            embassy_nrf::gpio::Output<'static>,
        >,
    >,
    284,
    76,
    43168,
>;

pub struct DisplayWrapper(pub &'static mut DisplayType);

impl rktk::drivers::interface::display::DisplayDriver for DisplayWrapper {
    type Color = embedded_graphics::pixelcolor::Rgb565;
    type Display = rktk_drivers_common::display::mipi_display::MipiDisplayWrapper<284, 76, 43168>;

    fn draw_target(&mut self) -> &mut Self::Display {
        self.0.draw_target()
    }

    async fn init(&mut self) -> Result<(), display_interface::DisplayError> {
        self.0.init().await?;
        self.0
            .display
            .set_color_format(display_driver::ColorFormat::RGB565)
            .await
            .map_err(|_| display_interface::DisplayError::BusWriteError)?;
        self.0
            .display
            .set_orientation(display_driver::Orientation::Deg270)
            .await
            .map_err(|_| display_interface::DisplayError::BusWriteError)?;
        Ok(())
    }

    async fn flush(&mut self) -> Result<(), display_interface::DisplayError> {
        self.0.flush().await
    }

    async fn clear(&mut self) -> Result<(), display_interface::DisplayError> {
        self.0.clear().await
    }

    async fn set_brightness(
        &mut self,
        brightness: u8,
    ) -> Result<(), display_interface::DisplayError> {
        self.0.set_brightness(brightness).await
    }

    async fn set_display_on(&mut self, on: bool) -> Result<(), display_interface::DisplayError> {
        self.0.set_display_on(on).await
    }
}

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

    // Turn on backlight
    let mut bl = Output::new(p.P0_22, Level::High, OutputDrive::Standard);
    bl.set_low();

    let reset_opt = display_driver::LCDResetOption::new_pin(Output::new(
        p.P1_06,
        Level::Low,
        OutputDrive::Standard,
    ));
    let panel = display_driver_st7789::St7789::<display::MyCustomPanel, _, _>::new(reset_opt);

    let mut spi_config = embassy_nrf::spim::Config::default();
    spi_config.frequency = embassy_nrf::spim::Frequency::M8;
    let spi = embassy_nrf::spim::Spim::new_txonly(p.TWISPI1, Irqs, p.P0_17, p.P0_20, spi_config);

    let cs = Output::new(p.P0_24, Level::High, OutputDrive::Standard);
    let device = embedded_hal_bus::spi::ExclusiveDevice::new(spi, cs, embassy_time::Delay).unwrap();

    let dc = Output::new(p.P1_00, Level::Low, OutputDrive::Standard);
    let bus = display_driver_spi::SpiDisplayBus::new(device, dc);

    let disp = display_driver::DisplayDriver::builder(bus, panel)
        .with_color_format(display_driver::ColorFormat::RGB565)
        .with_orientation(display_driver::Orientation::Deg270)
        .init(&mut embassy_time::Delay)
        .await
        .unwrap();

    let disp_drv = {
        static DISPLAY: static_cell::StaticCell<DisplayType> = static_cell::StaticCell::new();
        DISPLAY.init(MipiDisplayDriver::new(disp))
    };

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
        display: Some(DisplayWrapper(disp_drv)),
        split: dummy::split(),
        rgb: Some(rgb),
        ble_builder: dummy::ble_builder(),
        storage: Some(storage),
        debounce: dummy::debounce(), // Magnetic matrix handles its own "debounce" via RT logic
        encoder: Some(encoder),
    };

    let opts = rktk::config::RktkOpts {
        keymap,
        config: &rktk::config::DYNAMIC_CONFIG_FROM_FILE,
        display: rktk::task::display::color_bar::ColorBarDisplayConfig,
        rgb_layout: rktk::config::rgb::DummyLayout,
        hand: None,
    };

    rktk::task::start(spawner, drivers, create_empty_hooks(), opts).await;
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
