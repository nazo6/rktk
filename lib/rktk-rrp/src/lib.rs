#![cfg_attr(not(feature = "std"), no_std)]

pub mod endpoints;
pub use futures;
pub mod client;
pub mod server;
