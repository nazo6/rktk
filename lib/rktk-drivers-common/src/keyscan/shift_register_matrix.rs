use super::pressed::Pressed;
use embedded_hal::{digital::InputPin, spi::Operation};
use embedded_hal_async::spi::SpiDevice;
use rktk::{
    drivers::interface::keyscan::{Hand, KeyscanDriver},
    keymanager::state::KeyChangeEvent,
};

/// Matrix scanner using spi-like shift register such as 74HC595 as output pin.
///
/// NOTE: Currently, chained shift register is not supported and OUTPUT_PIN_COUNT must be number of 1 to 8.
pub struct ShiftRegisterMatrix<
    S: SpiDevice,
    IP: InputPin,
    const OUTPUT_PIN_COUNT: usize,
    const INPUT_PIN_COUNT: usize,
    const COLS: usize,
    const ROWS: usize,
> {
    row_shift_register: S,
    input_pins: [IP; INPUT_PIN_COUNT],
    pressed: Pressed<COLS, ROWS>,
    left_detect_key: (usize, usize),
    map_key: fn(usize, usize) -> Option<(usize, usize)>,
}

impl<
        S: SpiDevice,
        IP: InputPin,
        const OUTPUT_PIN_COUNT: usize,
        const INPUT_PIN_COUNT: usize,
        const COLS: usize,
        const ROWS: usize,
    > ShiftRegisterMatrix<S, IP, OUTPUT_PIN_COUNT, INPUT_PIN_COUNT, COLS, ROWS>
{
    /// Detect the hand and initialize the scanner.
    ///
    /// # Arguments
    /// - `row_shift_register`: SPI bus for the shift register.
    /// - `cols`: Column pins of the matrix. These pins should be pulled up.
    /// - `left_detect_key`: The (logical, not pin index) key position to detect the hand.
    /// - `map_key`: Function to map key position from pin number. This function must return
    ///    position within specified `COLS` and `ROWS`.
    ///    Signature: (row, col) -> Option<(row, col)>
    pub fn new(
        row_shift_register: S,
        input_pins: [IP; INPUT_PIN_COUNT],
        left_detect_key: (usize, usize),
        map_key: fn(usize, usize) -> Option<(usize, usize)>,
    ) -> Self {
        Self {
            row_shift_register,
            input_pins,
            left_detect_key,
            pressed: Pressed::new(),
            map_key,
        }
    }
}

impl<
        S: SpiDevice,
        IP: InputPin,
        const OUTPUT_PIN_COUNT: usize,
        const INPUT_PIN_COUNT: usize,
        const COLS: usize,
        const ROWS: usize,
    > KeyscanDriver for ShiftRegisterMatrix<S, IP, OUTPUT_PIN_COUNT, INPUT_PIN_COUNT, COLS, ROWS>
{
    // TODO: support async matrix
    async fn scan(&mut self, mut cb: impl FnMut(KeyChangeEvent)) {
        for output_idx in 0..OUTPUT_PIN_COUNT {
            let _ = self
                .row_shift_register
                .transaction(&mut [
                    Operation::DelayNs(1000),
                    Operation::Write(&[1 << output_idx]),
                ])
                .await;

            embassy_time::Timer::after_nanos(100).await;

            for (input_idx, input_pin) in self.input_pins.iter_mut().enumerate() {
                if let Some((row, col)) = (self.map_key)(input_idx, output_idx) {
                    if let Some(change) =
                        self.pressed
                            .set_pressed(input_pin.is_high().unwrap(), row, col)
                    {
                        cb(KeyChangeEvent {
                            row: row as u8,
                            col: col as u8,
                            pressed: change,
                        });
                    }
                }
            }
        }
    }

    async fn current_hand(&mut self) -> rktk::drivers::interface::keyscan::Hand {
        let mut hand = Hand::Right;
        let left_detect_key = self.left_detect_key;
        self.scan(|e| {
            if e.row == left_detect_key.0 as u8 && e.col == left_detect_key.1 as u8 {
                hand = Hand::Left;
            }
        })
        .await;
        hand
    }
}
