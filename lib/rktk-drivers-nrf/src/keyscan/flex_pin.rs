use embassy_nrf::{
    gpio::{Flex, OutputDrive, Pin, Pull as NrfPull},
    Peripheral,
};
pub use rktk_drivers_common::keyscan::duplex_matrix::ScanDir;
use rktk_drivers_common::keyscan::flex_pin::{FlexPin, Pull};

/// Wrapper over flex pin that implements rktk_drivers_common's [`FlexPin`] trait.
pub struct NrfFlexPin<'a> {
    pin: Flex<'a>,
    pull: NrfPull,
    drive: OutputDrive,
}

impl<'a> NrfFlexPin<'a> {
    pub fn new(pin: impl Peripheral<P = impl Pin> + 'a) -> Self {
        Self {
            pin: Flex::new(pin),
            pull: NrfPull::None,
            drive: OutputDrive::Standard,
        }
    }
}

impl FlexPin for NrfFlexPin<'_> {
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
