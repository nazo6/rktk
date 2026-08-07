#[macro_export]
macro_rules! create_spi {
    ($p:ident) => {{
        use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
        use embassy_sync::mutex::Mutex;
        use rktk_drivers_rp::mouse::paw3395;

        let mut spi_config = paw3395::recommended_spi_config();
        spi_config.frequency = 1_000_000;

        Mutex::<ThreadModeRawMutex, _>::new(embassy_rp::spi::Spi::new(
            $p.SPI0, $p.PIN_2, $p.PIN_3, $p.PIN_4, $p.DMA_CH0, $p.DMA_CH1, Irqs, spi_config,
        ))
    }};
}

#[macro_export]
macro_rules! driver_split {
    ($p:ident) => {{
        use embassy_rp::uart::BufferedUart;
        use rktk::singleton;
        use rktk_drivers_rp::split::uart_full_duplex::UartFullDuplexSplitDriver;

        let uart_config = embassy_rp::uart::Config::default();

        let (tx, rx) = ($p.PIN_0, $p.PIN_1);

        UartFullDuplexSplitDriver::new(BufferedUart::new(
            $p.UART0,
            tx,
            rx,
            Irqs,
            singleton!([0; 256], [u8; 256]),
            singleton!([0; 256], [u8; 256]),
            uart_config,
        ))
    }};
}

#[macro_export]
macro_rules! driver_display {
    ($p:ident) => {{
        use embassy_rp::i2c::I2c;
        use rktk_drivers_common::display::ssd1306;
        use rktk_drivers_common::display::ssd1306::Ssd1306Driver;
        use rktk_drivers_common::panic_utils;

        let mut display = Ssd1306Driver::new(
            I2c::new_async(
                $p.I2C1,
                $p.PIN_7,
                $p.PIN_6,
                Irqs,
                rktk_drivers_rp::display::ssd1306::recommended_i2c_config(),
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
        use embassy_rp::gpio::{Level, Output};
        #[cfg(feature = "paw3395")]
        use rktk_drivers_common::mouse::paw3395::Paw3395;
        #[cfg(feature = "pmw3360")]
        use rktk_drivers_common::mouse::pmw3360::Pmw3360;

        let ball_cs = Output::new($p.PIN_9, Level::High);

        #[cfg(feature = "paw3395")]
        {
            use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
            Paw3395::new(SpiDevice::new(&$spi, ball_cs), neg_common::PAW3395_CONFIG)
        }
        #[cfg(feature = "pmw3360")]
        {
            use rktk_drivers_common::spi::EmbassySpiDevice;
            let spi = EmbassySpiDevice::new(&$spi, ball_cs);
            Pmw3360::new(spi, Default::default())
        }
    }};
}

#[macro_export]
macro_rules! driver_keyscan {
    ($p:ident, $spi:ident) => {{
        use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
        use embassy_rp::gpio::{Input, Level, Output, Pull};
        use rktk_drivers_common::keyscan::shift_register_matrix::ShiftRegisterMatrix;

        let shift_register_cs = Output::new($p.PIN_8, Level::High);
        let shift_register_spi_device = SpiDevice::new(&$spi, shift_register_cs);

        ShiftRegisterMatrix::<_, _, _, 8, 5, 5, 8>::new(
            shift_register_spi_device,
            [
                Input::new($p.PIN_26, Pull::Down), // ROW0
                Input::new($p.PIN_22, Pull::Down), // ROW1
                Input::new($p.PIN_20, Pull::Down), // ROW2
                Input::new($p.PIN_23, Pull::Down), // ROW3
                Input::new($p.PIN_21, Pull::Down), // ROW4
            ],
            neg_common::translate_key_position,
            None,
        )
    }};
}

#[macro_export]
macro_rules! driver_encoder {
    ($p:ident) => {{
        use embassy_rp::gpio::{Input, Pull};
        use rktk_drivers_common::encoder::GeneralEncoder;

        GeneralEncoder::new([(
            Input::new($p.PIN_27, Pull::Down),
            Input::new($p.PIN_28, Pull::Down),
        )])
    }};
}

#[macro_export]
macro_rules! driver_system {
    () => {{ rktk_drivers_rp::system::RpSystemDriver }};
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
        use rktk_drivers_rp::rgb::ws2812_pio::Ws2812Pio;

        let pio = embassy_rp::pio::Pio::new($p.PIO0, Irqs);
        Ws2812Pio::<'_, 64, _>::new(pio, $p.PIN_5, $p.DMA_CH2, Irqs)
    }};
}
