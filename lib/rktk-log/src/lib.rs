#![no_std]

pub use rktk_log_macros::{derive_format_and_debug, maybe_derive_format};

#[cfg(feature = "defmtusb")]
pub mod defmtusb;
pub mod helper;
#[doc(hidden)]
pub mod macros;

#[doc(hidden)]
pub mod __reexports {
    #[cfg(feature = "defmt")]
    pub use defmt;

    #[cfg(feature = "log")]
    pub use log;

    pub use rktk_log_macros;
}
