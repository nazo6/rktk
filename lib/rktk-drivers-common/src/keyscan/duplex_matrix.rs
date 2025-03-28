//! (Japanese) duplex matrix scanner.

use super::{
    flex_pin::{FlexPin, Pull},
    pressed::Pressed,
};
use embassy_time::Duration;
use rktk::drivers::interface::keyscan::{KeyChangeEvent, KeyscanDriver};

/// How to change the wait for output pins
///
/// In rp2040, wait_for_{low,high} can be used for output mode, so just use `Pin`.
/// On the other hand, in nrf, this doesn't work and never returns.
/// In such case, set this to `Time` and will be fallback to just wait some time.
///
/// Default: Time(20us)
#[derive(Clone, Copy)]
pub enum OutputWait {
    Pin,
    Time(Duration),
}

/// Implementation of keyscan driver for [duplex matrix](https://kbd.news/The-Japanese-duplex-matrix-1391.html).
pub struct DuplexMatrixScanner<
    F: FlexPin,
    T: Fn(ScanDir, usize, usize) -> Option<(usize, usize)>,
    const ROW_PIN_COUNT: usize,
    const COL_PIN_COUNT: usize,
    const ROWS: usize,
    const COLS: usize,
> {
    rows: [F; ROW_PIN_COUNT],
    cols: [F; COL_PIN_COUNT],
    pressed: Pressed<ROWS, COLS>,
    output_wait: OutputWait,
    map_key_pos: T,
}

impl<
    F: FlexPin,
    T: Fn(ScanDir, usize, usize) -> Option<(usize, usize)>,
    const ROW_PIN_COUNT: usize,
    const COL_PIN_COUNT: usize,
    const ROWS: usize,
    const COLS: usize,
> DuplexMatrixScanner<F, T, ROW_PIN_COUNT, COL_PIN_COUNT, ROWS, COLS>
{
    /// Detect the hand and initialize the scanner.
    ///
    /// # Arguments
    /// - `rows`: Row pins of the matrix.
    /// - `cols`: Column pins of the matrix.
    /// - `output_awaitable`: Whether the output pins can be awaited for high/low. For more detail,
    ///   see [`OutputWait`].
    ///   Default: Time(20us)
    /// - `left_detect_key`: The (logical, not pin index) key position to detect the hand.
    /// - `translate_key_position`: Function to translate key position from pin number and scan direction to key
    ///   (scan direction, row, col) -> Option<(row, col)>
    pub fn new(
        rows: [F; ROW_PIN_COUNT],
        cols: [F; COL_PIN_COUNT],
        output_wait: Option<OutputWait>,
        translate_key_position: T,
    ) -> Self {
        Self {
            rows,
            cols,
            pressed: Pressed::new(),
            output_wait: output_wait.unwrap_or(OutputWait::Time(Duration::from_micros(20))),
            map_key_pos: translate_key_position,
        }
    }

    async fn wait_for_low(output_wait: OutputWait, pin: &mut F) {
        match output_wait {
            OutputWait::Pin => {
                pin.wait_for_low().await;
            }
            OutputWait::Time(duration) => {
                embassy_time::Timer::after(duration).await;
            }
        }
    }

    async fn wait_for_high(output_wait: OutputWait, pin: &mut F) {
        match output_wait {
            OutputWait::Pin => {
                pin.wait_for_high().await;
            }
            OutputWait::Time(duration) => {
                embassy_time::Timer::after(duration).await;
            }
        }
    }

    /// Scan the matrix using specific direction.
    ///
    /// # Arguments
    /// - `cb`: ([output pin index], [input pin index]) -> ()
    async fn scan_dir(
        outputs: &mut [F],
        inputs: &mut [F],
        output_wait: OutputWait,
        mut cb: impl FnMut(usize, usize, bool),
    ) {
        for output in outputs.iter_mut() {
            output.set_low();
        }
        for inputs in inputs.iter_mut() {
            inputs.set_pull(Pull::Down);
            inputs.set_as_input();
        }

        embassy_time::Timer::after_micros(20).await;

        for (o_i, output) in outputs.iter_mut().enumerate() {
            output.set_high();
            output.set_as_output();
            Self::wait_for_high(output_wait, output).await;

            for (i_i, input) in inputs.iter_mut().enumerate() {
                cb(o_i, i_i, input.is_high());
            }

            output.set_low();
            Self::wait_for_low(output_wait, output).await;
            output.set_as_input();
        }
    }
}

impl<
    F: FlexPin,
    T: Fn(ScanDir, usize, usize) -> Option<(usize, usize)>,
    const ROW_PIN_COUNT: usize,
    const COL_PIN_COUNT: usize,
    const ROWS: usize,
    const COLS: usize,
> KeyscanDriver for DuplexMatrixScanner<F, T, ROW_PIN_COUNT, COL_PIN_COUNT, ROWS, COLS>
{
    async fn scan(&mut self, mut cb: impl FnMut(KeyChangeEvent)) {
        Self::scan_dir(
            &mut self.rows,
            &mut self.cols,
            self.output_wait,
            |row_pin_idx, col_pin_idx, pressed| {
                if let Some((row, col)) =
                    (self.map_key_pos)(ScanDir::Row2Col, row_pin_idx, col_pin_idx)
                {
                    if let Some(change) = self.pressed.set_pressed(pressed, row, col) {
                        cb(KeyChangeEvent {
                            row: row as u8,
                            col: col as u8,
                            pressed: change,
                        });
                    }
                }
            },
        )
        .await;

        Self::scan_dir(
            &mut self.cols,
            &mut self.rows,
            self.output_wait,
            |col_pin_idx, row_pin_idx, pressed| {
                if let Some((row, col)) =
                    (self.map_key_pos)(ScanDir::Col2Row, row_pin_idx, col_pin_idx)
                {
                    if let Some(change) = self.pressed.set_pressed(pressed, row, col) {
                        cb(KeyChangeEvent {
                            row: row as u8,
                            col: col as u8,
                            pressed: change,
                        });
                    }
                }
            },
        )
        .await;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScanDir {
    Col2Row,
    Row2Col,
}
