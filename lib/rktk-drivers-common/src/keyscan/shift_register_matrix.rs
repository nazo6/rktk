use super::{pressed::Pressed, HandDetector};
use embedded_hal::{digital::InputPin, spi::Operation};
use embedded_hal_async::spi::SpiDevice;
use rktk::drivers::interface::keyscan::{Hand, KeyChangeEvent, KeyscanDriver};

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
    hand_detector: HandDetector,
    map_key: fn(usize, usize) -> Option<(usize, usize)>,
    scan_delay: embassy_time::Duration,
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
    /// Initialize the scanner.
    ///
    /// WARNING: Shift register is not actually spi so you should set proper spi mode.
    /// Also, the scan direction can be different depending spi mode.
    ///
    /// # Arguments
    /// * `row_shift_register`: SPI bus for the shift register used as output pin.
    /// * `input_pins`: Input pins to read the matrix.
    /// * `left_detect_key`: The (logical, not pin index) key position to detect the hand.
    /// * `map_key`: Function to map key position from pin number. This function must return
    ///   position within specified `COLS` and `ROWS`.
    ///   Signature: (row, col) -> Option<(row, col)>
    /// * `scan_delay`: Delay between output pin change and input read. This is executed for each
    ///   col/row so, this should be short enough to scan the matrix in a reasonable time.
    ///   Default: 5us
    pub fn new(
        row_shift_register: S,
        input_pins: [IP; INPUT_PIN_COUNT],
        hand_detector: HandDetector,
        map_key: fn(usize, usize) -> Option<(usize, usize)>,
        scan_delay: Option<embassy_time::Duration>,
    ) -> Self {
        Self {
            row_shift_register,
            input_pins,
            hand_detector,
            pressed: Pressed::new(),
            map_key,
            scan_delay: scan_delay.unwrap_or(embassy_time::Duration::from_micros(5)),
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

            embassy_time::Timer::after(self.scan_delay).await;

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
        match self.hand_detector {
            HandDetector::ByKey(d_col, d_row) => {
                let mut hand = Hand::Right;
                self.scan(|e| {
                    if e.row == d_col as u8 && e.col == d_row as u8 {
                        hand = Hand::Left;
                    }
                })
                .await;
                hand
            }
            HandDetector::Constant(hand) => hand,
        }
    }
}
