//! Encoder driver implementations.

use embassy_futures::select::{select, select_slice};
use embedded_hal::digital::InputPin;
use embedded_hal_async::digital::Wait;
use rktk::drivers::interface::encoder::{EncoderDirection, EncoderDriver};

const TRANSITIONS: [i8; 16] = [0, -1, 1, 0, 1, 0, 0, -1, -1, 0, 0, 1, 0, 1, -1, 0];

struct SingleEncoderState<PIN> {
    a: PIN,
    b: PIN,
    last_state: u8,
    accumulator: i8,
}

/// General encoder driver that can be used with any digital input pin.
pub struct GeneralEncoder<PIN: Wait + InputPin, const ENCODER_COUNT: usize> {
    encoders: [SingleEncoderState<PIN>; ENCODER_COUNT],
    resolution: i8,
}

impl<PIN: Wait + InputPin, const ENCODER_COUNT: usize> GeneralEncoder<PIN, ENCODER_COUNT> {
    pub fn new(encoders: [(PIN, PIN); ENCODER_COUNT]) -> Self {
        Self::new_with_resolution(encoders, 4)
    }

    pub fn new_with_resolution(encoders: [(PIN, PIN); ENCODER_COUNT], resolution: i8) -> Self {
        let encoders = encoders.map(|(mut a, mut b)| {
            let a_high = a.is_high().unwrap_or(false);
            let b_high = b.is_high().unwrap_or(false);
            let last_state = ((a_high as u8) << 1) | (b_high as u8);
            SingleEncoderState { a, b, last_state, accumulator: 0 }
        });

        Self { encoders, resolution: if resolution <= 0 { 1 } else { resolution } }
    }
}

impl<PIN: Wait + InputPin, const ENCODER_COUNT: usize> EncoderDriver
    for GeneralEncoder<PIN, ENCODER_COUNT>
{
    async fn read_wait(&mut self) -> (u8, EncoderDirection) {
        let resolution = self.resolution;
        let mut id = 0;
        let futures = self.encoders.each_mut().map(|enc| {
            let enc_id = id;
            id += 1;
            async move {
                loop {
                    let _ = select(enc.a.wait_for_any_edge(), enc.b.wait_for_any_edge()).await;

                    let a_high = enc.a.is_high().unwrap_or(false);
                    let b_high = enc.b.is_high().unwrap_or(false);
                    let curr_state = ((a_high as u8) << 1) | (b_high as u8);

                    let idx = ((enc.last_state << 2) | curr_state) as usize;
                    let delta = TRANSITIONS[idx];
                    enc.last_state = curr_state;

                    if delta != 0 {
                        enc.accumulator += delta;
                        if enc.accumulator >= resolution {
                            enc.accumulator -= resolution;
                            return (enc_id as u8, EncoderDirection::Clockwise);
                        } else if enc.accumulator <= -resolution {
                            enc.accumulator += resolution;
                            return (enc_id as u8, EncoderDirection::CounterClockwise);
                        }
                    }
                }
            }
        });

        select_slice(core::pin::pin!(futures)).await.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quadrature_clockwise_sequence() {
        // Standard CW sequence starting at rest (1,1): 11 -> 01 -> 00 -> 10 -> 11
        let cw_states = [0b11, 0b01, 0b00, 0b10, 0b11];
        let mut acc = 0;
        let mut last_state = cw_states[0];

        for &curr_state in &cw_states[1..] {
            let idx = ((last_state << 2) | curr_state) as usize;
            let delta = TRANSITIONS[idx];
            assert_eq!(delta, 1, "CW transition should add +1");
            acc += delta;
            last_state = curr_state;
        }

        assert_eq!(acc, 4, "Full CW turn should accumulate +4");
    }

    #[test]
    fn test_quadrature_counter_clockwise_sequence() {
        // Standard CCW sequence starting at rest (1,1): 11 -> 10 -> 00 -> 01 -> 11
        let ccw_states = [0b11, 0b10, 0b00, 0b01, 0b11];
        let mut acc = 0;
        let mut last_state = ccw_states[0];

        for &curr_state in &ccw_states[1..] {
            let idx = ((last_state << 2) | curr_state) as usize;
            let delta = TRANSITIONS[idx];
            assert_eq!(delta, -1, "CCW transition should subtract -1");
            acc += delta;
            last_state = curr_state;
        }

        assert_eq!(acc, -4, "Full CCW turn should accumulate -4");
    }

    #[test]
    fn test_quadrature_invalid_double_flip() {
        let idx = ((0b00 << 2) | 0b11) as usize;
        assert_eq!(TRANSITIONS[idx], 0, "Double-bit flip should yield 0 delta");
    }
}
