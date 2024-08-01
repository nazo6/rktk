use embassy_nrf::gpio::{Flex, OutputDrive, Pull};
use rktk::interface::keyscan::{Hand, KeyChangeEventOneHand, KeyscanDriver};

use super::pressed::Pressed;

pub struct DuplexMatrixScannerNrf<
    'd,
    const ROW_PIN_COUNT: usize,
    const COL_PIN_COUNT: usize,
    const COLS: usize,
    const ROWS: usize,
> {
    rows: [Flex<'d>; ROW_PIN_COUNT],
    cols: [Flex<'d>; COL_PIN_COUNT],
    pressed: Pressed<COLS, ROWS>,
    left_detect_jumper_key: (usize, usize),
}

impl<
        'd,
        const ROW_PIN_COUNT: usize,
        const COL_PIN_COUNT: usize,
        const COLS: usize,
        const ROWS: usize,
    > DuplexMatrixScannerNrf<'d, ROW_PIN_COUNT, COL_PIN_COUNT, COLS, ROWS>
{
    /// Detect the hand and initialize the scanner.
    pub fn new(
        rows: [Flex<'d>; ROW_PIN_COUNT],
        cols: [Flex<'d>; COL_PIN_COUNT],
        left_detect_jumper_key: (usize, usize),
    ) -> Self {
        Self {
            rows,
            cols,
            left_detect_jumper_key,
            pressed: Pressed::new(),
        }
    }
}

impl<
        'a,
        const ROW_PIN_COUNT: usize,
        const COL_PIN_COUNT: usize,
        const COLS: usize,
        const ROWS: usize,
    > KeyscanDriver for DuplexMatrixScannerNrf<'a, ROW_PIN_COUNT, COL_PIN_COUNT, COLS, ROWS>
{
    async fn scan(&mut self) -> heapless::Vec<KeyChangeEventOneHand, 16> {
        let mut events = heapless::Vec::new();

        rktk::print!("cb1");
        // col -> row scan
        {
            for row in self.rows.iter_mut() {
                row.set_as_input(Pull::Down);
            }

            for (j, col) in self.cols.iter_mut().enumerate() {
                // col -> rowスキャンではcol=3は該当キーなし
                if j == 3 {
                    continue;
                }

                col.set_as_output(OutputDrive::Standard);
                col.set_high();
                // col.wait_for_high().await;

                for (i, row) in self.rows.iter_mut().enumerate() {
                    if let Some(change) = self.pressed.set_pressed(row.is_high(), i as u8, j as u8)
                    {
                        let _ = events.push(KeyChangeEventOneHand {
                            row: i as u8,
                            col: j as u8,
                            pressed: change,
                        });
                    }
                }
                col.set_low();
                // col.wait_for_low().await;
                col.set_as_input(Pull::Down);
            }
        }
        rktk::print!("cb2");

        // row -> col scan
        {
            for col in self.cols.iter_mut() {
                col.set_as_input(Pull::Down);
            }

            for (i, row) in self.rows.iter_mut().enumerate() {
                row.set_as_output(OutputDrive::Standard);
                row.set_low();
                row.set_high();
                // row.wait_for_high().await;

                for (j, col) in self.cols.iter_mut().enumerate() {
                    // In left side, this is always high.
                    if (i, j + 3) == self.left_detect_jumper_key {
                        continue;
                    }

                    if let Some(change) =
                        self.pressed
                            .set_pressed(col.is_high(), i as u8, (j + 3) as u8)
                    {
                        let _ = events.push(KeyChangeEventOneHand {
                            row: i as u8,
                            col: (j + 3) as u8,
                            pressed: change,
                        });
                    }
                }

                row.set_low();
                // row.wait_for_low().await;
                row.set_as_input(Pull::Down);
            }
        }

        rktk::print!("cb3");

        rktk::print!("cb4");
        events
    }

    async fn current_hand(&mut self) -> rktk::interface::keyscan::Hand {
        rktk::print!("ch1");
        if self.left_detect_jumper_key.1 >= 4 {
            let row = &mut self.rows[self.left_detect_jumper_key.0];
            let col = &mut self.cols[self.left_detect_jumper_key.1 - 3];

            col.set_as_input(Pull::Down);

            row.set_high();
            row.set_as_output(OutputDrive::Standard);
            // row.wait_for_high().await;

            if col.is_high() {
                Hand::Left
            } else {
                Hand::Right
            }
        } else {
            panic!("Invalid left detect jumper config");
        }
    }
}

pub fn create_duplex_matrix<
    'a: 'a,
    const ROW_PIN_COUNT: usize,
    const COL_PIN_COUNT: usize,
    const ROWS: usize,
    const COLS: usize,
>(
    rows: [Flex<'a>; ROW_PIN_COUNT],
    cols: [Flex<'a>; COL_PIN_COUNT],
    left_detect_jumper_key: (usize, usize),
) -> impl KeyscanDriver + 'a {
    DuplexMatrixScannerNrf::<'a, ROW_PIN_COUNT, COL_PIN_COUNT, COLS, ROWS>::new(
        rows,
        cols,
        left_detect_jumper_key,
    )
}
