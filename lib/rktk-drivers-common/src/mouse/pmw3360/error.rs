use core::fmt::Debug;

use rktk::drivers::interface::{Error, ErrorKind};

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pmw3360Error<SpiError: Debug> {
    InvalidSignature,
    Spi(#[cfg_attr(feature = "defmt", defmt(Debug2Format))] SpiError),
    NotSupported,
}

impl<S: Debug> Error for Pmw3360Error<S> {
    fn kind(&self) -> ErrorKind {
        match self {
            Self::NotSupported => ErrorKind::NotSupported,
            _ => ErrorKind::Other,
        }
    }
}
