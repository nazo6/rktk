use super::pressed::Pressed;
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal_async::digital::Wait;
use rktk::drivers::interface::keyscan::{KeyChangeEvent, KeyscanDriver};

/// Matrix scanner
///
/// NOTE: `COLS` const generic is the number of columns. Though you can set this value same as
/// specified in [`rktk::::config::static_config::Keyboard::cols`], for split keyboard, this also can be set to
/// "columns of the half keyboard". By using this value, you can save memory usage.
pub struct Matrix<
    OP: OutputPin,
    IP: InputPin + Wait,
    T: Fn(usize, usize) -> Option<(usize, usize)>,
    const OUTPUT_PIN_COUNT: usize,
    const INPUT_PIN_COUNT: usize,
    const COLS: usize,
    const ROWS: usize,
> {
    output_pins: [OP; OUTPUT_PIN_COUNT],
    input_pins: [IP; INPUT_PIN_COUNT],
    pressed: Pressed<COLS, ROWS>,
    map_key_pos: T,
    scan_delay: embassy_time::Duration,
}

impl<
    OP: OutputPin,
    IP: InputPin + Wait,
    T: Fn(usize, usize) -> Option<(usize, usize)>,
    const OUTPUT_PIN_COUNT: usize,
    const INPUT_PIN_COUNT: usize,
    const COLS: usize,
    const ROWS: usize,
> Matrix<OP, IP, T, OUTPUT_PIN_COUNT, INPUT_PIN_COUNT, COLS, ROWS>
{
    /// Initialize the scanner.
    ///
    /// # Arguments
    /// - `output_pins`: Output pins to control the matrix.
    /// - `input_pins`: Input pins to read the matrix.
    /// - `left_detect_key`: The (logical, not pin index) key position to detect the hand.
    /// - `map_key`: Function to map key position from pin index. This function must return
    ///   position within specified `COLS` and `ROWS`.
    ///   Signature: (input_pin_idx, output_pin_idx) -> Option<(row, col)>
    /// * `scan_delay`: Delay between output pin change and input read. This is executed for each
    ///   col/row so, this should be short enough to scan the matrix in a reasonable time.
    ///   Default: 5us
    pub fn new(
        output_pins: [OP; OUTPUT_PIN_COUNT],
        input_pins: [IP; INPUT_PIN_COUNT],
        map_key_pos: T,
        scan_delay: Option<embassy_time::Duration>,
    ) -> Self {
        Self {
            output_pins,
            input_pins,
            pressed: Pressed::new(),
            map_key_pos,
            scan_delay: scan_delay.unwrap_or(embassy_time::Duration::from_micros(5)),
        }
    }
}

impl<
    OP: OutputPin,
    IP: InputPin + Wait,
    T: Fn(usize, usize) -> Option<(usize, usize)>,
    const OUTPUT_PIN_COUNT: usize,
    const INPUT_PIN_COUNT: usize,
    const COLS: usize,
    const ROWS: usize,
> KeyscanDriver for Matrix<OP, IP, T, OUTPUT_PIN_COUNT, INPUT_PIN_COUNT, COLS, ROWS>
{
    // TODO: support async matrix
    async fn scan(&mut self, mut cb: impl FnMut(KeyChangeEvent)) {
        for output_idx in 0..OUTPUT_PIN_COUNT {
            let _ = self.output_pins[output_idx].set_high();

            embassy_time::Timer::after(self.scan_delay).await;

            for (input_idx, input_pin) in self.input_pins.iter_mut().enumerate() {
                if let Some((row, col)) = (self.map_key_pos)(input_idx, output_idx) {
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
}
