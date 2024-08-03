use embedded_hal::{digital::OutputPin, spi::ErrorType};

#[derive(Debug)]
pub enum Pmw3360Error<SE: ErrorType, OP: OutputPin> {
    InvalidSignature,
    Spi(SE::Error),
    Gpio(OP::Error),
}
