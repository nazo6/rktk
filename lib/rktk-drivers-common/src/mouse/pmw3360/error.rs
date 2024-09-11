use core::fmt::Debug;

#[derive(Debug)]
pub enum Pmw3360Error<SpiError: Debug, GpioError: Debug> {
    InvalidSignature,
    Spi(SpiError),
    Gpio(GpioError),
}
