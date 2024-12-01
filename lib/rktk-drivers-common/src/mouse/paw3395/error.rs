use core::fmt::Debug;

#[derive(Debug, thiserror::Error)]
pub enum Paw3395Error<SpiError: Debug> {
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("SPI: {0:?}")]
    Spi(SpiError),
    #[error("General: {0}")]
    General(&'static str),
    #[error("Not supported")]
    NotSupported,
}
