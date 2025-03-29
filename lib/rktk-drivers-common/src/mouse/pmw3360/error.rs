use core::fmt::Debug;

#[derive(Debug, thiserror::Error)]
pub enum Pmw3360Error<SpiError: Debug> {
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("SPI: {0:?}")]
    Spi(SpiError),
    #[error("Not supported")]
    NotSupported,
}

#[cfg(feature = "defmt")]
impl<S: Debug> defmt::Format for Pmw3360Error<S> {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{}", self);
    }
}
