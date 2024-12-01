use core::fmt::Debug;

#[derive(Debug, thiserror::Error)]
pub enum Paw3395Error {
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("SPI")]
    Spi,
    #[error("General: {0}")]
    General(&'static str),
    #[error("Not supported")]
    NotSupported,
}
