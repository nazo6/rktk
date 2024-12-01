//! WS2812 PWM driver for nRF52
//! Based on https://github.com/embassy-rs/embassy/blob/main/examples/nrf52840/src/bin/pwm_sequence_ws2812b.rs

use core::convert::Infallible;

use bitvec::prelude::*;
use embassy_nrf::{
    gpio::{OutputDrive, Pin},
    pwm::{
        Config, Instance, Prescaler, SequenceLoad, SequencePwm, SingleSequenceMode, SingleSequencer,
    },
    Peripheral,
};
use embassy_time::Timer;
use rktk::drivers::interface::backlight::BacklightDriver;
use smart_leds::RGB8;

pub struct Ws2812Pwm<
    PWM: Instance,
    PWMP: Peripheral<P = PWM>,
    DATA: Pin,
    DATAP: Peripheral<P = DATA>,
> {
    pwm: PWMP,
    pin: DATAP,
}

const T1H: u16 = 0x8000 | 13;
const T0H: u16 = 0x8000 | 6;
const RES: u16 = 0x8000;

impl<PWM: Instance, PWMP: Peripheral<P = PWM>, DATA: Pin, DATAP: Peripheral<P = DATA>>
    Ws2812Pwm<PWM, PWMP, DATA, DATAP>
{
    pub fn new(pwm: PWMP, data: DATAP) -> Self {
        Self { pwm, pin: data }
    }
}

impl<PWM: Instance, PWMP: Peripheral<P = PWM>, DATA: Pin, DATAP: Peripheral<P = DATA>>
    BacklightDriver for Ws2812Pwm<PWM, PWMP, DATA, DATAP>
{
    type Error = Infallible;

    async fn write<const N: usize>(&mut self, colors: &[RGB8; N]) -> Result<(), Self::Error> {
        let mut pwm_config = Config::default();
        pwm_config.sequence_load = SequenceLoad::Common;
        pwm_config.prescaler = Prescaler::Div1; // 16MHz
        pwm_config.max_duty = 20; // 1.25us (1s / 16Mhz * 20)
        pwm_config.ch0_drive = OutputDrive::HighDrive;
        let Ok(mut seq_pwm) = SequencePwm::new_1ch(&mut self.pwm, &mut self.pin, pwm_config) else {
            // TODO: Handle error
            return Ok(());
        };

        let mut words = heapless::Vec::<u16, 1024>::from_slice(&[RES; 100]).unwrap();

        for color in colors {
            for bit in color
                .g
                .view_bits::<Msb0>()
                .iter()
                .chain(color.r.view_bits())
                .chain(color.b.view_bits())
            {
                words.push(if *bit { T1H } else { T0H }).unwrap();
            }
        }

        let seq_config = embassy_nrf::pwm::SequenceConfig::default();
        let sequencer = SingleSequencer::new(&mut seq_pwm, words.as_slice(), seq_config);
        let _ = sequencer.start(SingleSequenceMode::Times(1));

        // Wait for a long time is important. Otherwise, the PWM will be stopped before the
        // sequence is finished.
        Timer::after_millis(50).await;

        Ok(())
    }
}
