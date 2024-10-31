use core::fmt::Debug;

#[derive(Debug)]
pub enum Pmw3360Error<SpiError: Debug> {
    InvalidSignature,
    Spi(SpiError),
}
