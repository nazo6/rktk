#![cfg_attr(not(feature = "std-client"), no_std)]

#[cfg(feature = "std-client")]
pub mod client;
pub mod endpoints;
pub mod server;

pub mod __reexports {
    pub use futures;
    pub use heapless;
    pub use postcard;
}
