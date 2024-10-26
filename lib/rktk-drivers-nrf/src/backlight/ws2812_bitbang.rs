//! WS2812 bitbang driver for nRF52
//! FIXME: This is not working

use bitvec::{prelude::*, view::BitView};
use embassy_nrf::gpio::{AnyPin, Output, OutputDrive};
use embassy_time::Timer;
use rktk::interface::backlight::BacklightDriver;
use smart_leds::RGB8;

pub struct Ws2812Bitbang<'d> {
    output: Output<'d>,
}

impl<'d> Ws2812Bitbang<'d> {
    pub fn new(pin: AnyPin) -> Self {
        let output = Output::new(pin, embassy_nrf::gpio::Level::High, OutputDrive::Standard);
        Self { output }
    }

    async fn write_0(&mut self) {
        self.output.set_high();
        Timer::after_nanos(200).await;
        self.output.set_low();
        Timer::after_nanos(900).await;
    }
    async fn write_1(&mut self) {
        self.output.set_high();
        Timer::after_nanos(500).await;
        self.output.set_low();
        Timer::after_nanos(360).await;
    }
    async fn write_rst(&mut self) {
        self.output.set_low();
        Timer::after_micros(150).await;
    }
}

impl<'d> BacklightDriver for Ws2812Bitbang<'d> {
    async fn write<const N: usize>(&mut self, colors: &[RGB8; N]) {
        self.write_rst().await;
        for color in colors {
            for bit in color
                .g
                .view_bits::<Msb0>()
                .iter()
                .chain(color.r.view_bits())
                .chain(color.b.view_bits())
            {
                if *bit {
                    self.write_1().await;
                } else {
                    self.write_0().await;
                }
            }
        }
        self.output.set_high();
    }
}
