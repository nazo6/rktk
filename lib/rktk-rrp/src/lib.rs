#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "client")]
pub mod client;
pub mod endpoints;
#[cfg(feature = "server")]
pub mod server;

#[doc(hidden)]
pub mod __reexports {
    pub use futures;
    pub use heapless;
    pub use postcard;
}
