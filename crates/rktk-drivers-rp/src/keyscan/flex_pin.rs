use embassy_rp::{
    Peri,
    gpio::{Flex, Pin},
};
use rktk_drivers_common::keyscan::flex_pin::{FlexPin, Pull};

/// Wrapper over flex pin that implements rktk_drivers_common's [`FlexPin`] trait.
pub struct RpFlexPin<'a>(Flex<'a>);

impl<'a> RpFlexPin<'a> {
    pub fn new(pin: Peri<'a, impl Pin>) -> Self {
        Self(Flex::new(pin))
    }
}

impl FlexPin for RpFlexPin<'_> {
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

    fn is_low(&self) -> bool {
        self.0.is_low()
    }

    async fn wait_for_high(&mut self) {
        self.0.wait_for_high().await;
    }

    async fn wait_for_low(&mut self) {
        self.0.wait_for_low().await;
    }

    fn set_pull(&mut self, pull: Pull) {
        self.0.set_pull(match pull {
            Pull::Up => embassy_rp::gpio::Pull::Up,
            Pull::Down => embassy_rp::gpio::Pull::Down,
        });
    }
}
