use display_interface::DisplayError;
use embassy_nrf::{
    gpio::Pin,
    interrupt::typelevel::Binding,
    pac::Interrupt::SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0,
    peripherals::TWISPI0,
    twim::{Config, Frequency, InterruptHandler, Twim},
    Peripheral,
};
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};

const TEXT_STYLE: MonoTextStyle<'static, BinaryColor> = MonoTextStyleBuilder::new()
    .font(&FONT_6X10)
    .text_color(BinaryColor::On)
    .background_color(BinaryColor::Off)
    .build();

/// SSD1306 OLED module
pub struct Oled<'a> {
    display: Ssd1306<
        I2CInterface<Twim<'a, TWISPI0>>,
        DisplaySize128x32,
        BufferedGraphicsMode<DisplaySize128x32>,
    >,
}

#[allow(dead_code)]
impl<'a> Oled<'a> {
    pub fn new(
        twim: impl Peripheral<P = TWISPI0> + 'a,
        _irq: impl Binding<
                <embassy_nrf::peripherals::TWISPI0 as embassy_nrf::twis::Instance>::Interrupt,
                InterruptHandler<TWISPI0>,
            > + 'a,
        sda: impl Peripheral<P = impl Pin> + 'a,
        scl: impl Peripheral<P = impl Pin> + 'a,
    ) -> (Self, bool) {
        let mut config = Config::default();
        config.frequency = Frequency::K400;
        let interface = I2CDisplayInterface::new(Twim::new(twim, _irq, sda, scl, config));

        let mut display = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
        let success = display.init().is_ok();

        (Self { display }, success)
    }

    pub const fn calculate_point(col: i32, row: i32) -> Point {
        Point::new((col - 1) * 6, (row - 1) * 10)
    }

    pub async fn clear(&mut self) -> Result<(), DisplayError> {
        self.display.clear_buffer();
        self.display.flush_async().await
    }

    pub async fn update_text(&mut self, text: &str, point: Point) -> Result<(), DisplayError> {
        Text::with_baseline(text, point, TEXT_STYLE, Baseline::Top)
            .draw(&mut self.display)
            .unwrap();

        self.display.flush_async().await
    }

    pub fn draw_text_blocking(&mut self, text: &str) -> Result<(), DisplayError> {
        self.display.clear_buffer();

        Text::with_baseline(text, Point::zero(), TEXT_STYLE, Baseline::Top)
            .draw(&mut self.display)
            .unwrap();

        self.display.flush()
    }

    pub fn update_text_blocking(&mut self, text: &str, point: Point) -> Result<(), DisplayError> {
        Text::with_baseline(text, point, TEXT_STYLE, Baseline::Top)
            .draw(&mut self.display)
            .unwrap();
        self.display.flush()
    }
}
