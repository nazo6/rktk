use core::fmt::Debug;

use rktk::drivers::interface::{Error, ErrorKind};

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Paw3395Error<SpiError: Debug> {
    InvalidSignature,
    Spi(#[cfg_attr(feature = "defmt", defmt(Debug2Format))] SpiError),
    General(&'static str),
    NotSupported,
}

impl<S: Debug> Error for Paw3395Error<S> {
    fn kind(&self) -> ErrorKind {
        match self {
            Self::NotSupported => ErrorKind::NotSupported,
            _ => ErrorKind::Other,
        }
    }
}
