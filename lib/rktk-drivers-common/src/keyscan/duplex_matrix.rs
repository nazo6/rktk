use super::pressed::Pressed;
use rktk::{
    interface::keyscan::{Hand, KeyscanDriver},
    keymanager::state::KeyChangeEvent,
};

pub enum Pull {
    Up,
    Down,
}

#[allow(async_fn_in_trait)]
pub trait FlexPin {
    fn set_as_input(&mut self);
    fn set_as_output(&mut self);
    fn set_low(&mut self);
    fn set_high(&mut self);
    fn is_high(&self) -> bool;
    async fn wait_for_high(&mut self);
    async fn wait_for_low(&mut self);
    fn set_pull(&mut self, pull: Pull);
}

pub struct DuplexMatrixScanner<
    'a,
    F: FlexPin + 'a,
    const ROW_PIN_COUNT: usize,
    const COL_PIN_COUNT: usize,
    const COLS: usize,
    const ROWS: usize,
> {
    _phantom: core::marker::PhantomData<&'a ()>,
    rows: [F; ROW_PIN_COUNT],
    cols: [F; COL_PIN_COUNT],
    pressed: Pressed<COLS, ROWS>,
    left_detect_jumper_key: (usize, usize),
    /// In rp2040, wait_for_{low,high} can be used for output mode.
    /// On the other hand, in nrf, this doesn't work and never returns.
    /// In such case, set this to false and will be fallback to just wait some time
    output_awaitable: bool,
}

impl<
        'a,
        F: FlexPin + 'a,
        const ROW_PIN_COUNT: usize,
        const COL_PIN_COUNT: usize,
        const COLS: usize,
        const ROWS: usize,
    > DuplexMatrixScanner<'a, F, ROW_PIN_COUNT, COL_PIN_COUNT, COLS, ROWS>
{
    /// Detect the hand and initialize the scanner.
    pub fn new(
        rows: [F; ROW_PIN_COUNT],
        cols: [F; COL_PIN_COUNT],
        left_detect_jumper_key: (usize, usize),
        output_awaitable: bool,
    ) -> Self {
        Self {
            _phantom: core::marker::PhantomData,
            rows,
            cols,
            left_detect_jumper_key,
            pressed: Pressed::new(),
            output_awaitable,
        }
    }

    async fn wait_for_low(output_awaitable: bool, pin: &mut F) {
        if output_awaitable {
            pin.wait_for_low().await;
        } else {
            embassy_time::Timer::after_ticks(1).await;
        }
    }

    async fn wait_for_high(output_awaitable: bool, pin: &mut F) {
        if output_awaitable {
            pin.wait_for_high().await;
        } else {
            embassy_time::Timer::after_ticks(1).await;
        }
    }

    async fn scan_with_cb(&mut self, mut cb: impl FnMut(KeyChangeEvent)) {
        // col -> row scan
        {
            for row in self.rows.iter_mut() {
                row.set_pull(Pull::Down);
                row.set_as_input();
            }

            for (j, col) in self.cols.iter_mut().enumerate() {
                // col -> rowスキャンではcol=3は該当キーなし
                if j == 3 {
                    continue;
                }

                col.set_high();
                col.set_as_output();
                Self::wait_for_high(self.output_awaitable, col).await;

                for (i, row) in self.rows.iter_mut().enumerate() {
                    if let Some(change) = self.pressed.set_pressed(row.is_high(), i as u8, j as u8)
                    {
                        cb(KeyChangeEvent {
                            row: i as u8,
                            col: j as u8,
                            pressed: change,
                        });
                    }
                }
                col.set_low();
                Self::wait_for_low(self.output_awaitable, col).await;
                col.set_as_input();
            }
        }

        // row -> col scan
        {
            for col in self.cols.iter_mut() {
                col.set_as_input();
                col.set_pull(Pull::Down);
            }

            for (i, row) in self.rows.iter_mut().enumerate() {
                row.set_high();
                row.set_as_output();
                Self::wait_for_high(self.output_awaitable, row).await;

                for (j, col) in self.cols.iter_mut().enumerate() {
                    // In left side, this is always high.
                    if (i, j + 3) == self.left_detect_jumper_key {
                        continue;
                    }

                    if let Some(change) =
                        self.pressed
                            .set_pressed(col.is_high(), i as u8, (j + 3) as u8)
                    {
                        cb(KeyChangeEvent {
                            row: i as u8,
                            col: (j + 3) as u8,
                            pressed: change,
                        })
                    }
                }

                row.set_low();
                Self::wait_for_low(self.output_awaitable, row).await;
                row.set_as_input();
            }
        }
    }
}

impl<
        'a,
        F: FlexPin + 'a,
        const ROW_PIN_COUNT: usize,
        const COL_PIN_COUNT: usize,
        const COLS: usize,
        const ROWS: usize,
    > KeyscanDriver for DuplexMatrixScanner<'a, F, ROW_PIN_COUNT, COL_PIN_COUNT, COLS, ROWS>
{
    async fn scan(&mut self) -> heapless::Vec<KeyChangeEvent, 32> {
        let mut events = heapless::Vec::new();
        self.scan_with_cb(|e| {
            events.push(e).ok();
        })
        .await;
        // rktk::print!("{:?}                    ", self.pressed);
        events
    }

    async fn current_hand(&mut self) -> rktk::interface::keyscan::Hand {
        if self.left_detect_jumper_key.1 >= 4 {
            let row = &mut self.rows[self.left_detect_jumper_key.0];
            let col = &mut self.cols[self.left_detect_jumper_key.1 - 3];

            col.set_as_input();
            col.set_pull(Pull::Down);

            row.set_as_output();
            row.set_high();
            Self::wait_for_high(self.output_awaitable, row).await;

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
