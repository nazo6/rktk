use embedded_hal::digital::OutputPin;
use rktk::drivers::interface::magnetic::Multiplexer;

/// SN74LV4051 8-channel analog multiplexer.
pub struct Sn74lv4051<S0: OutputPin, S1: OutputPin, S2: OutputPin> {
    s0: S0,
    s1: S1,
    s2: S2,
}

impl<S0: OutputPin, S1: OutputPin, S2: OutputPin> Sn74lv4051<S0, S1, S2> {
    pub fn new(s0: S0, s1: S1, s2: S2) -> Self {
        Self { s0, s1, s2 }
    }
}

impl<E: core::fmt::Debug, S0: OutputPin<Error = E>, S1: OutputPin<Error = E>, S2: OutputPin<Error = E>>
    Multiplexer for Sn74lv4051<S0, S1, S2>
{
    type Error = E;

    async fn select(&mut self, channel: u8) -> Result<(), Self::Error> {
        if channel & 0b001 != 0 {
            self.s0.set_high()?;
        } else {
            self.s0.set_low()?;
        }

        if channel & 0b010 != 0 {
            self.s1.set_high()?;
        } else {
            self.s1.set_low()?;
        }

        if channel & 0b100 != 0 {
            self.s2.set_high()?;
        } else {
            self.s2.set_low()?;
        }

        Ok(())
    }
}
