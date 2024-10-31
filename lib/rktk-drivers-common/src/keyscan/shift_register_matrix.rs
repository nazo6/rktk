use super::pressed::Pressed;
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal_async::spi::SpiDevice;
use rktk::{
    interface::keyscan::{Hand, KeyscanDriver},
    keymanager::state::KeyChangeEvent,
};

pub struct ShiftRegisterMatrix<
    S: SpiDevice,
    OP: OutputPin,
    IP: InputPin,
    const OUTPUT_PIN_COUNT: usize,
    const INPUT_PIN_COUNT: usize,
    const COLS: usize,
    const ROWS: usize,
> {
    row_shift_register: S,
    shift_register_cs: OP,
    cols: [IP; INPUT_PIN_COUNT],
    pressed: Pressed<COLS, ROWS>,
    left_detect_key: (usize, usize),
    map_key: fn(usize, usize) -> Option<(usize, usize)>,
}

impl<
        S: SpiDevice,
        OP: OutputPin,
        IP: InputPin,
        const OUTPUT_PIN_COUNT: usize,
        const INPUT_PIN_COUNT: usize,
        const COLS: usize,
        const ROWS: usize,
    > ShiftRegisterMatrix<S, OP, IP, OUTPUT_PIN_COUNT, INPUT_PIN_COUNT, COLS, ROWS>
{
    /// Detect the hand and initialize the scanner.
    ///
    /// # Arguments
    /// - `row_shift_register`: SPI bus for the shift register.
    /// - `shift_register_cs`: CS pin for the shift register.
    /// - `cols`: Column pins of the matrix. These pins should be pulled up.
    /// - `left_detect_key`: The (logical, not pin index) key position to detect the hand.
    /// - `translate_key_position`: Function to translate key position from pin number and scan direction to key
    ///    (scan direction, row, col) -> Option<(row, col)>
    pub fn new(
        row_shift_register: S,
        shift_register_cs: OP,
        cols: [IP; INPUT_PIN_COUNT],
        left_detect_key: (usize, usize),
        map_key: fn(usize, usize) -> Option<(usize, usize)>,
    ) -> Self {
        Self {
            row_shift_register,
            shift_register_cs,
            cols,
            left_detect_key,
            pressed: Pressed::new(),
            map_key,
        }
    }

    async fn scan_with_cb(&mut self, mut cb: impl FnMut(KeyChangeEvent)) {
        for row_idx in 0..OUTPUT_PIN_COUNT {
            let _ = self.shift_register_cs.set_low();
            let _ = self
                .row_shift_register
                .transfer_in_place(&mut [1 << row_idx])
                .await;
            let _ = self.shift_register_cs.set_high();

            for (col_idx, col_pin) in self.cols.iter_mut().enumerate() {
                if let Some((row, col)) = (self.map_key)(row_idx, col_idx) {
                    if let Some(change) =
                        self.pressed
                            .set_pressed(col_pin.is_high().unwrap(), row, col)
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
}

impl<
        S: SpiDevice,
        OP: OutputPin,
        IP: InputPin,
        const ROW_PIN_COUNT: usize,
        const COL_PIN_COUNT: usize,
        const COLS: usize,
        const ROWS: usize,
    > KeyscanDriver for ShiftRegisterMatrix<S, OP, IP, ROW_PIN_COUNT, COL_PIN_COUNT, COLS, ROWS>
{
    async fn scan(&mut self) -> heapless::Vec<KeyChangeEvent, 32> {
        let mut events = heapless::Vec::new();
        self.scan_with_cb(|e| {
            events.push(e).ok();
        })
        .await;
        events
    }

    async fn current_hand(&mut self) -> rktk::interface::keyscan::Hand {
        if self
            .scan()
            .await
            .iter()
            .any(|e| e.row == self.left_detect_key.1 as u8 && e.col == self.left_detect_key.1 as u8)
        {
            Hand::Left
        } else {
            Hand::Right
        }
    }
}
