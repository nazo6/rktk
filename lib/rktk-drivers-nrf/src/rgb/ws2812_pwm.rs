//! WS2812 PWM driver for nRF52
//! Based on https://github.com/embassy-rs/embassy/blob/main/examples/nrf52840/src/bin/pwm_sequence_ws2812b.rs

use core::convert::Infallible;

use bitvec::prelude::*;
use embassy_nrf::{
    Peripheral,
    gpio::{OutputDrive, Pin},
    pwm::{
        Config, Instance, Prescaler, SequenceLoad, SequencePwm, SingleSequenceMode, SingleSequencer,
    },
};
use embassy_time::Timer;
use rktk::drivers::interface::rgb::{LedRgb, RgbDriver};

/// WS2812 NeoPixel driver using PWM on nRF.
///
/// ## Note about `BUFFER_SIZE`
/// Buffer size should be large enough to hold the number of pixels you want to control.
/// It should be larger than `led_count * 24 + 100` to ensure enough space for the sequence.
///
/// Even buffer size is not enough, pixels will be written, but the driver will log a warning.
pub struct Ws2812Pwm<
    const BUFFER_SIZE: usize,
    PWM: Instance,
    PWMP: Peripheral<P = PWM>,
    DATA: Pin,
    DATAP: Peripheral<P = DATA>,
> {
    pwm: PWMP,
    pin: DATAP,
}

const T1H: u16 = 0x8000 | 13;
const T0H: u16 = 0x8000 | 7;
const RES: u16 = 0x8000;

impl<
    const BUFFER_SIZE: usize,
    PWM: Instance + 'static,
    PWMP: Peripheral<P = PWM> + 'static,
    DATA: Pin,
    DATAP: Peripheral<P = DATA> + 'static,
> Ws2812Pwm<BUFFER_SIZE, PWM, PWMP, DATA, DATAP>
{
    pub fn new(pwm: PWMP, data: DATAP) -> Self {
        Self { pwm, pin: data }
    }
}

impl<
    const BUFFER_SIZE: usize,
    PWM: Instance + 'static,
    PWMP: Peripheral<P = PWM> + 'static,
    DATA: Pin,
    DATAP: Peripheral<P = DATA> + 'static,
> RgbDriver for Ws2812Pwm<BUFFER_SIZE, PWM, PWMP, DATA, DATAP>
{
    type Error = Infallible;

    async fn write<I: IntoIterator<Item = LedRgb<u8>>>(
        &mut self,
        pixels: I,
    ) -> Result<(), Self::Error> {
        let mut pwm_config = Config::default();
        pwm_config.sequence_load = SequenceLoad::Common;
        pwm_config.prescaler = Prescaler::Div1; // 16MHz
        pwm_config.max_duty = 20; // 1.25us (1s / 16Mhz * 20)
        pwm_config.ch0_drive = OutputDrive::HighDrive;
        let Ok(mut seq_pwm) = SequencePwm::new_1ch(&mut self.pwm, &mut self.pin, pwm_config) else {
            // TODO: Handle error
            return Ok(());
        };

        let mut words = heapless::Vec::<u16, BUFFER_SIZE>::from_slice(&[RES; 100]).unwrap();

        'outer: for color in pixels {
            for bit in color[1]
                .view_bits::<Msb0>()
                .iter()
                .chain(color[0].view_bits())
                .chain(color[2].view_bits())
            {
                if words.push(if *bit { T1H } else { T0H }).is_err() {
                    rktk_log::warn!("WS2812Pwm buffer size is not enough. Increase BUFFER_SIZE.");
                    break 'outer;
                }
            }
        }

        let seq_config = embassy_nrf::pwm::SequenceConfig::default();
        let sequencer = SingleSequencer::new(&mut seq_pwm, words.as_slice(), seq_config);
        let _ = sequencer.start(SingleSequenceMode::Times(1));

        // Wait for a long time is important. Otherwise, the PWM will be stopped before the
        // sequence is finished.
        Timer::after_millis(5).await;

        Ok(())
    }
}
