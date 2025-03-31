#![doc = include_str!("../README.md")]
//! ## Feature flags
#![doc = document_features::document_features!()]
#![no_std]
#![cfg_attr(doc, feature(doc_auto_cfg))]

pub mod debounce;
pub mod display;
pub mod encoder;
pub mod keyscan;
pub mod mouse;
pub mod panic_utils;
pub mod storage;
#[cfg(feature = "trouble")]
pub mod trouble;
pub mod usb;

#[cfg(feature = "defmt-timestamp")]
defmt::timestamp!("{=u64:us}", embassy_time::Instant::now().as_micros());
