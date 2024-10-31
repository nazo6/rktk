use core::fmt::Debug;

#[derive(Debug)]
pub enum Paw3395Error<SpiError: Debug> {
    InvalidSignature,
    Spi(SpiError),
    General(&'static str),
}
