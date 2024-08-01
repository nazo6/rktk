use embassy_rp::gpio::Flex;
use rktk::interface::keyscan::KeyscanDriver;
use rktk_drivers_common::keyscan::duplex_matrix::{DuplexMatrixScanner, FlexPin};

struct FlexWrap<'a>(Flex<'a>);

impl<'a> FlexPin for FlexWrap<'a> {
    fn set_as_input(&mut self) {
        self.0.set_as_input();
    }

    fn set_as_output(&mut self) {
        self.0.set_as_output();
    }

    fn set_low(&mut self) {
        self.0.set_low();
    }

    fn set_high(&mut self) {
        self.0.set_high();
    }

    fn is_high(&self) -> bool {
        self.0.is_high()
    }

    async fn wait_for_high(&mut self) {
        self.0.wait_for_high().await;
    }

    async fn wait_for_low(&mut self) {
        self.0.wait_for_low().await;
    }

    fn set_pull(&mut self, pull: rktk_drivers_common::keyscan::duplex_matrix::Pull) {
        self.0.set_pull(match pull {
            rktk_drivers_common::keyscan::duplex_matrix::Pull::Up => embassy_rp::gpio::Pull::Up,
            rktk_drivers_common::keyscan::duplex_matrix::Pull::Down => embassy_rp::gpio::Pull::Down,
        });
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
    let rows = rows.map(FlexWrap);
    let cols = cols.map(FlexWrap);
    DuplexMatrixScanner::<'a, FlexWrap<'a>, ROW_PIN_COUNT, COL_PIN_COUNT, COLS, ROWS>::new(
        rows,
        cols,
        left_detect_jumper_key,
        true,
    )
}
