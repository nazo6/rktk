#![doc = include_str!("../README.md")]
#![no_std]

pub mod debounce;
pub mod display;
pub mod encoder;
pub mod keyscan;
pub mod mouse;
pub mod panic_utils;
pub mod storage;
pub mod usb;

#[cfg(feature = "defmt-timestamp")]
defmt::timestamp!("{=u64:us}", embassy_time::Instant::now().as_micros());
