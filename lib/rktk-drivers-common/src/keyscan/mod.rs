//! Key scanning driver implementations.

use embassy_time::Duration;
use embedded_hal::digital::{InputPin, OutputPin};
use rktk::interface::Hand;

pub mod duplex_matrix;
pub mod flex_pin;
pub mod matrix;
mod pressed;
pub mod shift_register_matrix;

/// Utility function that takes an output pin and an input pin and determines the move based on the result of the input pin when the output pin is high.
///
/// * `output`: Output pin
/// * `input`: Input pin
/// * `input_delay`: Time from output pin high to read (default: 10ms)
/// * `high_hand`: Which move is judged when the input pin is high? (default: Left)
pub async fn detect_hand_from_matrix<E, O: OutputPin<Error = E>, I: InputPin<Error = E>>(
    mut output: O,
    mut input: I,
    input_delay: Option<Duration>,
    high_hand: Option<Hand>,
) -> Result<Hand, E> {
    let high_hand = high_hand.unwrap_or(Hand::Left);

    output.set_high()?;
    embassy_time::Timer::after(input_delay.unwrap_or(Duration::from_millis(10))).await;
    let hand = if input.is_high()? {
        high_hand
    } else {
        high_hand.other()
    };
    output.set_low()?;

    Ok(hand)
}
