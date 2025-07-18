#[macro_export]
macro_rules! create_spi {
    ($p:ident) => {{
        use embassy_nrf::gpio::OutputDrive;
        use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
        use embassy_sync::mutex::Mutex;
        use rktk_drivers_nrf::mouse::paw3395;

        let mut spi_config = paw3395::recommended_spi_config();
        spi_config.sck_drive = OutputDrive::Standard;
        spi_config.mosi_drive = OutputDrive::Standard;
        spi_config.frequency = embassy_nrf::spim::Frequency::K250;

        Mutex::<ThreadModeRawMutex, _>::new(embassy_nrf::spim::Spim::new(
            $p.SPI2, Irqs, $p.P0_17, $p.P0_22, $p.P0_20, spi_config,
        ))
    }};
}

#[macro_export]
macro_rules! driver_split {
    ($p:ident) => {{
        use embassy_nrf::buffered_uarte::BufferedUarte;
        use rktk::singleton;
        use rktk_drivers_nrf::split::uart_full_duplex::UartFullDuplexSplitDriver;

        let uarte_config = embassy_nrf::uarte::Config::default();

        #[cfg(feature = "reversed-split-pins")]
        let (sp1, sp2) = ($p.P0_06, $p.P0_08);
        #[cfg(not(feature = "reversed-split-pins"))]
        let (sp1, sp2) = ($p.P0_08, $p.P0_06);

        UartFullDuplexSplitDriver::new(BufferedUarte::new(
            $p.UARTE0,
            $p.TIMER1,
            $p.PPI_CH0,
            $p.PPI_CH1,
            $p.PPI_GROUP0,
            sp1,
            sp2,
            Irqs,
            uarte_config,
            singleton!([0; 256], [u8; 256]),
            singleton!([0; 256], [u8; 256]),
        ))
    }};
}

#[macro_export]
macro_rules! driver_display {
    ($p:ident) => {{
        use embassy_nrf::twim::Twim;
        use rktk_drivers_common::display::ssd1306;
        use rktk_drivers_common::display::ssd1306::Ssd1306Driver;
        use rktk_drivers_common::panic_utils;

        let mut display = Ssd1306Driver::new(
            Twim::new(
                $p.TWISPI0,
                Irqs,
                $p.P1_00,
                $p.P0_11,
                rktk_drivers_nrf::display::ssd1306::recommended_i2c_config(),
                &mut [],
            ),
            ssd1306::prelude::DisplaySize128x32,
            ssd1306::prelude::DisplayRotation::Rotate90,
        );
        panic_utils::display_message_if_panicked(&mut display).await;
        display
    }};
}

#[macro_export]
macro_rules! driver_mouse {
    ($p:ident, $spi:ident) => {{
        use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
        use embassy_nrf::gpio::{Output, OutputDrive};
        #[cfg(feature = "paw3395")]
        use rktk_drivers_common::mouse::paw3395::Paw3395;
        #[cfg(feature = "pmw3360")]
        use rktk_drivers_common::mouse::pmw3360::Pmw3360;

        let ball_cs = Output::new(
            $p.P1_06,
            embassy_nrf::gpio::Level::High,
            OutputDrive::Standard,
        );
        let ball_spi_device = SpiDevice::new(&$spi, ball_cs);

        #[cfg(feature = "paw3395")]
        {
            Paw3395::new(ball_spi_device, misc::PAW3395_CONFIG)
        }
        #[cfg(feature = "pmw3360")]
        {
            Pmw3360::new(ball_spi_device)
        }
    }};
}

#[macro_export]
macro_rules! driver_keyscan {
    ($p:ident, $spi:ident) => {{
        use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
        use embassy_nrf::gpio::{Input, Output, OutputDrive, Pull};
        use rktk_drivers_common::keyscan::shift_register_matrix::ShiftRegisterMatrix;

        let shift_register_cs = Output::new(
            $p.P1_04,
            embassy_nrf::gpio::Level::High,
            OutputDrive::Standard,
        );
        let shift_register_spi_device = SpiDevice::new(&$spi, shift_register_cs);

        ShiftRegisterMatrix::<_, _, _, 8, 5, 5, 8>::new(
            shift_register_spi_device,
            [
                Input::new($p.P1_15, Pull::Down), // ROW0
                Input::new($p.P1_13, Pull::Down), // ROW1
                Input::new($p.P1_11, Pull::Down), // ROW2
                Input::new($p.P0_10, Pull::Down), // ROW3
                Input::new($p.P0_09, Pull::Down), // ROW4
            ],
            misc::translate_key_position,
            None,
        )
    }};
}

#[macro_export]
macro_rules! driver_encoder {
    ($p:ident) => {{
        use embassy_nrf::gpio::{Input, Pull};
        use rktk_drivers_common::encoder::GeneralEncoder;

        GeneralEncoder::new([(
            Input::new($p.P0_02, Pull::Down),
            Input::new($p.P0_29, Pull::Down),
        )])
    }};
}

#[macro_export]
macro_rules! driver_system {
    ($p:ident) => {{
        use embassy_nrf::gpio::{Level, Output, OutputDrive};
        use rktk_drivers_nrf::system::NrfSystemDriver;

        let vcc_cutoff = (
            Output::new($p.P0_13, Level::High, OutputDrive::Standard),
            Level::Low,
        );
        NrfSystemDriver::new(Some(vcc_cutoff))
    }};
}

#[macro_export]
macro_rules! driver_debounce {
    () => {{
        rktk_drivers_common::debounce::EagerDebounceDriver::new(
            embassy_time::Duration::from_millis(10),
            true,
        )
    }};
}

#[macro_export]
macro_rules! driver_rgb {
    ($p:ident) => {{
        use rktk_drivers_nrf::rgb::ws2812_pwm::Ws2812Pwm;

        Ws2812Pwm::<1024, _, _>::new($p.PWM0, $p.P0_24)
    }};
}
