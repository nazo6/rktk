//! Magnetic switch driver interface.

/// ADC driver interface for magnetic switches.
pub trait Adc {
    /// Error type for ADC operations.
    type Error: core::fmt::Debug;

    /// Reads the ADC value.
    /// Returns a 16-bit value. If the ADC has lower resolution, it should be scaled to 16-bit.
    async fn read(&mut self) -> Result<u16, Self::Error>;
}

/// Analog multiplexer driver interface.
pub trait Multiplexer {
    /// Error type for multiplexer operations.
    type Error: core::fmt::Debug;

    /// Selects the channel.
    async fn select(&mut self, channel: u8) -> Result<(), Self::Error>;
}
