#![cfg_attr(not(feature = "std"), no_std)]

pub mod client;
pub mod endpoints;
pub mod server;

pub mod __reexports {
    pub use futures;
    pub use heapless;
    pub use postcard;
}
