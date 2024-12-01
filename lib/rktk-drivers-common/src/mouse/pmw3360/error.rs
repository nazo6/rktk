use core::fmt::Debug;

#[derive(Debug, thiserror::Error)]
pub enum Pmw3360Error {
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("SPI")]
    Spi,
    #[error("Not supported")]
    NotSupported,
}
