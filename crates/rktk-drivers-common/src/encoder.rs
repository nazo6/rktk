//! Encoder driver implementations.

use embassy_futures::select::{Either, select, select_slice};
use embassy_time::Timer;
use embedded_hal::digital::InputPin;
use embedded_hal_async::digital::Wait;
use rktk::drivers::interface::encoder::{EncoderDirection, EncoderDriver};

/// General encoder driver that can be used with any digital input pin.
pub struct GeneralEncoder<PIN: Wait + InputPin, const ENCODER_COUNT: usize> {
    encoders: [(PIN, PIN); ENCODER_COUNT],
}

impl<PIN: Wait + InputPin, const ENCODER_COUNT: usize> GeneralEncoder<PIN, ENCODER_COUNT> {
    pub fn new(encoders: [(PIN, PIN); ENCODER_COUNT]) -> Self {
        Self { encoders }
    }
}

impl<PIN: Wait + InputPin, const ENCODER_COUNT: usize> EncoderDriver
    for GeneralEncoder<PIN, ENCODER_COUNT>
{
    async fn read_wait(&mut self) -> (u8, EncoderDirection) {
        let mut futures = self
            .encoders
            .iter_mut()
            .enumerate()
            .map(|(i, (a, b))| async move {
                let dir = match select(a.wait_for_any_edge(), b.wait_for_any_edge()).await {
                    Either::First(_) => {
                        Timer::after_ticks(100).await;
                        if a.is_high().unwrap() ^ b.is_high().unwrap() {
                            EncoderDirection::Clockwise
                        } else {
                            EncoderDirection::CounterClockwise
                        }
                    }
                    Either::Second(_) => {
                        Timer::after_ticks(100).await;
                        if a.is_high().unwrap() ^ b.is_high().unwrap() {
                            EncoderDirection::CounterClockwise
                        } else {
                            EncoderDirection::Clockwise
                        }
                    }
                };
                (i as u8, dir)
            })
            .collect::<heapless::Vec<_, ENCODER_COUNT>>();

        select_slice(core::pin::pin!(&mut futures)).await.0
    }
}
