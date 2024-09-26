use core::fmt::Debug;

#[derive(Debug)]
pub enum Paw3395Error<SpiError: Debug, GpioError: Debug> {
    InvalidSignature,
    Spi(SpiError),
    Gpio(GpioError),
    General(&'static str),
}
