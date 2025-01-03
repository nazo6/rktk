//! (Japanese) duplex matrix scanner.

use super::{
    flex_pin::{FlexPin, Pull},
    pressed::Pressed,
    HandDetector,
};
use rktk::{
    drivers::interface::keyscan::{Hand, KeyscanDriver},
    keymanager::state::KeyChangeEvent,
};

/// Implementation of keyscan driver for [duplex matrix](https://kbd.news/The-Japanese-duplex-matrix-1391.html).
pub struct DuplexMatrixScanner<
    F: FlexPin,
    const ROW_PIN_COUNT: usize,
    const COL_PIN_COUNT: usize,
    const COLS: usize,
    const ROWS: usize,
> {
    rows: [F; ROW_PIN_COUNT],
    cols: [F; COL_PIN_COUNT],
    pressed: Pressed<COLS, ROWS>,
    output_awaitable: bool,
    hand_detector: HandDetector,
    translate_key_position: fn(ScanDir, usize, usize) -> Option<(usize, usize)>,
}

impl<
        F: FlexPin,
        const ROW_PIN_COUNT: usize,
        const COL_PIN_COUNT: usize,
        const COLS: usize,
        const ROWS: usize,
    > DuplexMatrixScanner<F, ROW_PIN_COUNT, COL_PIN_COUNT, COLS, ROWS>
{
    /// Detect the hand and initialize the scanner.
    ///
    /// # Arguments
    /// - `rows`: Row pins of the matrix.
    /// - `cols`: Column pins of the matrix.
    /// - `output_awaitable`: Whether the output pins can be awaited for high/low.
    ///    In rp2040, wait_for_{low,high} can be used for output mode.
    ///    On the other hand, in nrf, this doesn't work and never returns.
    ///    In such case, set this to false and will be fallback to just wait some time
    /// - `left_detect_key`: The (logical, not pin index) key position to detect the hand.
    /// - `translate_key_position`: Function to translate key position from pin number and scan direction to key
    ///    (scan direction, row, col) -> Option<(row, col)>
    pub fn new(
        rows: [F; ROW_PIN_COUNT],
        cols: [F; COL_PIN_COUNT],
        hand_detector: HandDetector,
        output_awaitable: bool,
        translate_key_position: fn(ScanDir, usize, usize) -> Option<(usize, usize)>,
    ) -> Self {
        Self {
            rows,
            cols,
            hand_detector,
            pressed: Pressed::new(),
            output_awaitable,
            translate_key_position,
        }
    }

    async fn wait_for_low(output_awaitable: bool, pin: &mut F) {
        if output_awaitable {
            pin.wait_for_low().await;
        } else {
            embassy_time::Timer::after_micros(20).await;
        }
    }

    async fn wait_for_high(output_awaitable: bool, pin: &mut F) {
        if output_awaitable {
            pin.wait_for_high().await;
        } else {
            embassy_time::Timer::after_micros(20).await;
        }
    }

    /// Scan the matrix using specific direction.
    ///
    /// # Arguments
    /// - `cb`: ([output pin index], [input pin index]) -> ()
    async fn scan_dir(
        outputs: &mut [F],
        inputs: &mut [F],
        output_awaitable: bool,
        mut cb: impl FnMut(usize, usize, bool),
    ) {
        for output in outputs.iter_mut() {
            output.set_low();
        }
        for inputs in inputs.iter_mut() {
            inputs.set_pull(Pull::Down);
            inputs.set_as_input();
        }

        embassy_time::Timer::after_micros(10).await;

        for (o_i, output) in outputs.iter_mut().enumerate() {
            output.set_high();
            output.set_as_output();
            Self::wait_for_high(output_awaitable, output).await;

            for (i_i, input) in inputs.iter_mut().enumerate() {
                cb(o_i, i_i, input.is_high());
            }

            output.set_low();
            Self::wait_for_low(output_awaitable, output).await;
            output.set_as_input();
        }
    }
}

impl<
        F: FlexPin,
        const ROW_PIN_COUNT: usize,
        const COL_PIN_COUNT: usize,
        const COLS: usize,
        const ROWS: usize,
    > KeyscanDriver for DuplexMatrixScanner<F, ROW_PIN_COUNT, COL_PIN_COUNT, COLS, ROWS>
{
    async fn scan(&mut self, mut cb: impl FnMut(KeyChangeEvent)) {
        Self::scan_dir(
            &mut self.rows,
            &mut self.cols,
            self.output_awaitable,
            |row_pin_idx, col_pin_idx, pressed| {
                if let Some((row, col)) =
                    (self.translate_key_position)(ScanDir::Row2Col, row_pin_idx, col_pin_idx)
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
            self.output_awaitable,
            |col_pin_idx, row_pin_idx, pressed| {
                if let Some((row, col)) =
                    (self.translate_key_position)(ScanDir::Col2Row, row_pin_idx, col_pin_idx)
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScanDir {
    Col2Row,
    Row2Col,
}
