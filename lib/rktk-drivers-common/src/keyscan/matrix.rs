use super::pressed::Pressed;
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal_async::digital::Wait;
use rktk::{
    drivers::interface::keyscan::{Hand, KeyscanDriver},
    keymanager::state::KeyChangeEvent,
};

/// Matrix scanner
pub struct Matrix<
    OP: OutputPin,
    IP: InputPin + Wait,
    const OUTPUT_PIN_COUNT: usize,
    const INPUT_PIN_COUNT: usize,
    const COLS: usize,
    const ROWS: usize,
> {
    output_pins: [OP; OUTPUT_PIN_COUNT],
    input_pins: [IP; INPUT_PIN_COUNT],
    pressed: Pressed<COLS, ROWS>,
    left_detect_key: (usize, usize),
    map_key: fn(usize, usize) -> Option<(usize, usize)>,
}

impl<
        OP: OutputPin,
        IP: InputPin + Wait,
        const OUTPUT_PIN_COUNT: usize,
        const INPUT_PIN_COUNT: usize,
        const COLS: usize,
        const ROWS: usize,
    > Matrix<OP, IP, OUTPUT_PIN_COUNT, INPUT_PIN_COUNT, COLS, ROWS>
{
    /// Initialize the scanner.
    ///
    /// # Arguments
    /// - `output_pins`: Output pins to control the matrix.
    /// - `input_pins`: Input pins to read the matrix.
    /// - `left_detect_key`: The (logical, not pin index) key position to detect the hand.
    /// - `map_key`: Function to map key position from pin number. This function must return
    ///    position within specified `COLS` and `ROWS`.
    ///    Signature: (row, col) -> Option<(row, col)>
    pub fn new(
        output_pins: [OP; OUTPUT_PIN_COUNT],
        input_pins: [IP; INPUT_PIN_COUNT],
        left_detect_key: (usize, usize),
        map_key: fn(usize, usize) -> Option<(usize, usize)>,
    ) -> Self {
        Self {
            output_pins,
            input_pins,
            left_detect_key,
            pressed: Pressed::new(),
            map_key,
        }
    }
}

impl<
        OP: OutputPin,
        IP: InputPin + Wait,
        const OUTPUT_PIN_COUNT: usize,
        const INPUT_PIN_COUNT: usize,
        const COLS: usize,
        const ROWS: usize,
    > KeyscanDriver for Matrix<OP, IP, OUTPUT_PIN_COUNT, INPUT_PIN_COUNT, COLS, ROWS>
{
    // TODO: support async matrix
    async fn scan(&mut self, mut cb: impl FnMut(KeyChangeEvent)) {
        for output_idx in 0..OUTPUT_PIN_COUNT {
            let _ = self.output_pins[output_idx].set_high();

            embassy_time::Timer::after_micros(10).await;

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

            let _ = self.output_pins[output_idx].set_low();
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
