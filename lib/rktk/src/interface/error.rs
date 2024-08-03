use core::fmt::{self, Formatter};

pub enum RktkError {
    GeneralError(&'static str),
    NotSupported,
}
impl core::fmt::Debug for RktkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RktkError::GeneralError(s) => write!(f, "{}", s),
            RktkError::NotSupported => write!(f, "NotSupported"),
        }
    }
}
