#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "client"), not(feature = "server")))]
compile_error!("At least, one of the `client` or `server` features should be enabled");

#[cfg(feature = "client")]
pub mod client;
pub mod endpoints;
#[cfg(feature = "server")]
pub mod server;
pub mod transport;

mod macros;

#[cfg(test)]
mod tests;
