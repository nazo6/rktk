use embassy_nrf::gpio::{Flex, OutputDrive, Pull};
use rktk::interface::keyscan::KeyscanDriver;
use rktk_drivers_common::keyscan::duplex_matrix::{DuplexMatrixScanner, FlexPin};

struct FlexWrap<'a> {
    pin: Flex<'a>,
    pull: Pull,
    drive: OutputDrive,
}

impl<'a> FlexPin for FlexWrap<'a> {
    fn set_as_input(&mut self) {
        let pull = match self.pull {
            Pull::Up => Pull::Up,
            Pull::Down => Pull::Down,
            Pull::None => Pull::None,
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

    async fn wait_for_high(&mut self) {
        self.pin.wait_for_high().await;
    }

    async fn wait_for_low(&mut self) {
        self.pin.wait_for_low().await;
    }

    fn set_pull(&mut self, pull: rktk_drivers_common::keyscan::duplex_matrix::Pull) {
        self.pull = match pull {
            rktk_drivers_common::keyscan::duplex_matrix::Pull::Up => Pull::Up,
            rktk_drivers_common::keyscan::duplex_matrix::Pull::Down => Pull::Down,
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
    left_detect_jumper_key: (usize, usize),
) -> impl KeyscanDriver + 'a {
    let rows = rows.map(|pin| FlexWrap {
        pin,
        pull: Pull::None,
        drive: OutputDrive::Standard,
    });
    let cols = cols.map(|pin| FlexWrap {
        pin,
        pull: Pull::None,
        drive: OutputDrive::Standard,
    });
    DuplexMatrixScanner::<'a, FlexWrap<'a>, ROW_PIN_COUNT, COL_PIN_COUNT, COLS, ROWS>::new(
        rows,
        cols,
        left_detect_jumper_key,
    )
}
