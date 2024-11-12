use embassy_nrf::gpio::{Flex, OutputDrive, Pull as NrfPull};
use rktk::interface::keyscan::KeyscanDriver;
pub use rktk_drivers_common::keyscan::duplex_matrix::ScanDir;
use rktk_drivers_common::keyscan::{
    duplex_matrix::DuplexMatrixScanner,
    flex_pin::{FlexPin, Pull},
};

struct FlexWrap<'a> {
    pin: Flex<'a>,
    pull: NrfPull,
    drive: OutputDrive,
}

impl FlexPin for FlexWrap<'_> {
    fn set_as_input(&mut self) {
        #[allow(clippy::needless_match)]
        let pull = match self.pull {
            NrfPull::Up => NrfPull::Up,
            NrfPull::Down => NrfPull::Down,
            NrfPull::None => NrfPull::None,
        };
        self.pin.set_as_input(pull);
    }

    fn set_as_output(&mut self) {
        self.pin.set_as_output(self.drive);
    }

    fn set_low(&mut self) {
        self.pin.set_low();
    }

    fn set_high(&mut self) {
        self.pin.set_high();
    }

    fn is_high(&self) -> bool {
        self.pin.is_high()
    }

    fn is_low(&self) -> bool {
        self.pin.is_low()
    }

    async fn wait_for_high(&mut self) {
        self.pin.wait_for_high().await;
    }

    async fn wait_for_low(&mut self) {
        self.pin.wait_for_low().await;
    }

    fn set_pull(&mut self, pull: Pull) {
        self.pull = match pull {
            Pull::Up => NrfPull::Up,
            Pull::Down => NrfPull::Down,
        };
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
    left_detect_key: (usize, usize),
    translate_key_position: fn(ScanDir, usize, usize) -> Option<(usize, usize)>,
) -> impl KeyscanDriver + 'a {
    let rows = rows.map(|pin| FlexWrap {
        pin,
        pull: NrfPull::None,
        drive: OutputDrive::HighDrive,
    });
    let cols = cols.map(|pin| FlexWrap {
        pin,
        pull: NrfPull::None,
        drive: OutputDrive::HighDrive,
    });
    DuplexMatrixScanner::<FlexWrap<'a>, ROW_PIN_COUNT, COL_PIN_COUNT, COLS, ROWS>::new(
        rows,
        cols,
        left_detect_key,
        false,
        translate_key_position,
    )
}
